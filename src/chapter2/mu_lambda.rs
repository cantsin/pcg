use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc};
use rand::{Rng, thread_rng};
use threadpool::{ThreadPool};

use chapter2::evaluation::{EvaluationFn};
use chapter2::genotype::{Genotype};
use chapter2::statistics::{Statistic};
use util::util::{shuffle};

pub struct MuLambda<G: Genotype> {
    threads: usize,
    iterations: u32,
    current_iteration: u32,
    mu: usize,     // number to keep
    lambda: usize, // number to generate
    mutation: f64, // mutation of genotype to mutate (between 0.0 and 1.0)
    genotype: G,
    evaluations: Arc<Vec<(EvaluationFn, f64)>>,
}

impl<G: Genotype + Clone + Send + 'static> MuLambda<G> {
    pub fn new(threads: usize,
               iterations: u32,
               mu: usize,
               lambda: usize,
               mutation: f64,
               genotype: G,
               funcs: Vec<EvaluationFn>,
               weights: Vec<f64>) -> MuLambda<G> {
        MuLambda {
            threads: threads,
            iterations: iterations,
            current_iteration: 0,
            mu: mu,
            lambda: lambda,
            mutation: mutation,
            genotype: genotype,
            evaluations: Arc::new(funcs.into_iter().zip(weights.iter().cloned()).collect()),
        }
    }

    pub fn run(&mut self) -> Vec<(G, Statistic)> {
        let total = self.mu + self.lambda;
        let mut rng = thread_rng();
        let mut primer: Vec<(G, Statistic)> = (0..total).map(|_| {
            (self.genotype.initialize(&mut rng), Statistic::empty())
        }).collect();
        // for each iteration except the last, do a full generation life-cycle.
        while self.current_iteration < self.iterations - 1 {
            primer = self.iterate(&mut rng, primer.as_mut_slice(), self.current_iteration);
            primer = self.prune(primer);
            self.current_iteration += 1;
        }
        // one last iteration without the pruning.
        primer = self.iterate(&mut rng, primer.as_mut_slice(), self.current_iteration);
        primer.clone()
    }

    fn iterate<R: Rng>(&self, rng: &mut R, primer: &mut [(G, Statistic)], iteration: u32) -> Vec<(G, Statistic)> {
        // shuffle the population
        shuffle(rng, primer);
        // calculate the fitness for each individual (in a separate thread)
        let n = primer.len();
        let pool = ThreadPool::new(self.threads);
        let (tx, rx): (Sender<(G, Statistic)>, Receiver<(G, Statistic)>) = mpsc::channel();
        for &(ref adult, _) in primer.iter() {
            let individual = adult.clone();
            let sender = tx.clone();
            let fns = self.evaluations.clone();
            pool.execute(move || {
                let dungeon = individual.generate();
                let fitness = individual.evaluate(&dungeon, &fns[..]);
                let statistic = Statistic::new(iteration, fitness);
                sender.send((individual, statistic)).unwrap();
            });
        }
        let mut colony: Vec<(G, Statistic)> = (0..n).map(|_| rx.recv().unwrap()).collect();
        // sort by fitness
        colony.sort_by(|&(_, ref i1), &(_, ref i2)| {
            match i1.fitness.partial_cmp(&i2.fitness) {
                Some(ordering) => ordering,
                None => panic!("{:?} and {:?} could not be ordered.", i1.fitness, i2.fitness)
            }
        });
        colony
    }

    fn prune(&self, generation: Vec<(G, Statistic)>) -> Vec<(G, Statistic)> {
        // keep mu
        let mut survivors: Vec<(G, Statistic)> = generation.iter().map(|v| v.clone()).take(self.mu).collect();
        // add the next generation
        let next_generation: Vec<(G, Statistic)> = generation.into_iter().map(|(ref mut individual, _)| {
            let mut new_rng = thread_rng();
            let mut baby = individual.clone();
            baby.mutate(&mut new_rng, self.mutation);
            (baby, Statistic::empty())
        }).skip(self.mu).collect();
        survivors.push_all(&next_generation[..]);
        survivors
    }
}
