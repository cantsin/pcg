use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc, TaskPool};
use rand::{Rng, thread_rng};

use evaluation::{EvaluationFn};
use genotype::{Genotype};
use statistics::{Statistic};
use util::{shuffle};

// TODO?
//type Result = (Genotype, Statistic);

pub struct MuLambda<G: Genotype> {
    threads: usize,
    iterations: usize,
    current_iteration: usize,
    mu: usize,     // number to keep
    lambda: usize, // number to generate
    mutation: f64, // mutation of genotype to mutate (between 0.0 and 1.0)
    genotype: G,
    evaluations: Arc<Vec<EvaluationFn>>
}

impl<G: Genotype + Clone + Send + 'static> MuLambda<G> {
    pub fn new(threads: usize,
               iterations: usize,
               mu: usize,
               lambda: usize,
               mutation: f64,
               genotype: G,
               funcs: Vec<EvaluationFn>) -> MuLambda<G> {
        MuLambda {
            threads: threads,
            iterations: iterations,
            current_iteration: 0,
            mu: mu,
            lambda: lambda,
            mutation: mutation,
            genotype: genotype,
            evaluations: Arc::new(funcs)
        }
    }

    pub fn evaluate(&mut self) -> Vec<(G, Statistic)> {
        let total = self.mu + self.lambda;
        let mut rng = thread_rng();
        let mut primer: Vec<(G, Statistic)> = range(0, total).map(|_| {
            (self.genotype.initialize(&mut rng), Statistic::empty())
        }).collect();
        while self.current_iteration < self.iterations {
            primer = self.iterate(&mut rng, primer.as_mut_slice(), self.current_iteration);
            self.current_iteration += 1;
        }
        primer.clone()
    }

    fn iterate<R: Rng>(&self, rng: &mut R, primer: &mut [(G, Statistic)], iteration: usize) -> Vec<(G, Statistic)> {
        // shuffle the population
        shuffle(rng, primer);
        // calculate the fitness for each individual (in a separate thread)
        let n = primer.len();
        let pool = TaskPool::new(self.threads);
        let (tx, rx): (Sender<(G, Statistic)>, Receiver<(G, Statistic)>) = mpsc::channel();
        for &(ref adult, _) in primer.iter() {
            let individual = adult.clone();
            let sender = tx.clone();
            let fns = self.evaluations.clone();
            pool.execute(move || {
                let dungeon = individual.generate();
                let fitness = individual.evaluate(&dungeon, fns.as_slice());
                let statistic = Statistic::new(iteration as u32, fitness);
                sender.send((individual, statistic)).unwrap();
            });
        }
        let mut colony: Vec<(G, Statistic)> = range(0, n).map(|_| rx.recv().unwrap()).collect();
        // sort by fitness
        colony.sort_by(|&(_, ref f1), &(_, ref f2)| {
            match f1.fitness.partial_cmp(&f2.fitness) {
                Some(ordering) => ordering,
                None => panic!(format!("{:?} and {:?} could not be ordered.", f1.fitness, f2.fitness))
            }
        });
        // keep mu
        let mut survivors: Vec<(G, Statistic)> = colony.iter().map(|v| v.clone()).take(self.mu).collect();
        // add the next generation
        let next_generation: Vec<(G, Statistic)> = colony.into_iter().map(|(ref mut individual, _)| {
            let mut new_rng = thread_rng();
            let mut baby = individual.clone();
            baby.mutate(&mut new_rng, self.mutation);
            (baby, Statistic::empty())
        }).skip(self.mu).collect();
        survivors.push_all(next_generation.as_slice());
        survivors
    }
}
