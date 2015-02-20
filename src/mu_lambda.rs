use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc, TaskPool};
use std::collections::{HashMap};
use rand::{Rng, thread_rng};

use evaluation::{EvaluationFn};
use genotype::{GenoType};
use util::{shuffle};

pub struct MuLambda<G: GenoType> {
    threads: usize,
    iterations: usize,
    current_iteration: usize,
    mu: usize,     // number to keep
    lambda: usize, // number to generate
    genotype: G,
    evaluations: Arc<Vec<EvaluationFn>>
}

impl<G: GenoType + Clone + Send> MuLambda<G> {
    pub fn new(threads: usize,
               iterations: usize,
               mu: usize,
               lambda: usize,
               genotype: G,
               funcs: Vec<EvaluationFn>) -> MuLambda<G> {
        MuLambda {
            threads: threads,
            iterations: iterations,
            current_iteration: 0,
            mu: mu,
            lambda: lambda,
            genotype: genotype,
            evaluations: Arc::new(funcs)
        }
    }

    pub fn evaluate(&mut self) -> Vec<G> {
        let total = self.mu + self.lambda;
        let mut primer: Vec<G> = range(0, total).map(|_| self.genotype.clone()).collect();
        let mut rng = thread_rng();
        while self.current_iteration < self.iterations {
            primer = self.iterate(&mut rng, primer.as_mut_slice(), self.current_iteration);
            self.current_iteration += 1;
        }
        primer.clone()
    }

    fn iterate<R: Rng>(&self, rng: &mut R, primer: &mut [G], iteration: usize) -> Vec<G> {
        // shuffle the population
        shuffle(rng, primer);
        // calculate the fitness for each individual (in a separate thread)
        let n = primer.len();
        let pool = TaskPool::new(self.threads);
        let (tx, rx): (Sender<(G, f64)>, Receiver<(G, f64)>) = mpsc::channel();
        for adult in primer {
            let mut individual = adult.clone();
            let sender = tx.clone();
            let fns = self.evaluations.clone();
            pool.execute(move || {
                let mut new_rng = thread_rng();
                let dungeon = individual.generate(&mut new_rng);
                let fitness = individual.evaluate(&dungeon, fns.as_slice());
                let mut stats: HashMap<String, f64> = HashMap::new();
                stats.insert(String::from_str("iteration"), iteration as f64);
                stats.insert(String::from_str("ranking"), fitness);
                individual.statistics(&stats);
                sender.send((individual, fitness)).unwrap();
            });
        }
        let mut colony: Vec<(G, f64)> = range(0, n).map(|_| rx.recv().unwrap()).collect();
        // sort by fitness
        colony.sort_by(|&(_, f1), &(_, f2)| {
            match f1.partial_cmp(&f2) {
                Some(ordering) => ordering,
                None => panic!(format!("{:?} and {:?} could not be ordered.", f1, f2))
            }
        });
        // keep mu
        let mut survivors: Vec<G> = colony.iter().map(|&(ref i, _)| i.clone()).take(self.mu).collect();
        // add the next generation
        let laggards: Vec<G> = colony.iter().map(|&(ref i, _)| i.clone()).skip(self.mu).collect();
        let next_generation: Vec<G> = laggards.iter().map(|individual| {
            let mut new_rng = thread_rng();
            let mut baby = individual.clone();
            baby.mutate(&mut new_rng);
            baby.generate(&mut new_rng);
            let mut stats: HashMap<String, f64> = HashMap::new();
            stats.insert(String::from_str("iteration"), (iteration + 1) as f64);
            baby.statistics(&stats);
            baby
        }).collect();
        survivors.push_all(next_generation.as_slice());
        survivors
    }
}
