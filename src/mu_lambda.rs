use rand::{Rng, thread_rng};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread::Thread;

use evaluation::{EvaluationFn};
use genotype::{GenoType};
use util::{shuffle};

pub struct MuLambda<G: GenoType> {
    iterations: usize,
    current_iteration: usize,
    mu: usize,     // number to keep
    lambda: usize, // number to generate
    genotype: G,
    evaluations: Vec<EvaluationFn>
}

impl<G: GenoType + Clone + Send> MuLambda<G> {
    pub fn new(iterations: usize, mu: usize, lambda: usize, genotype: G, funcs: Vec<EvaluationFn>) -> MuLambda<G> {
        MuLambda {
            iterations: iterations,
            current_iteration: 0,
            mu: mu,
            lambda: lambda,
            genotype: genotype,
            evaluations: funcs
        }
    }

    pub fn evaluate(&mut self) -> Vec<G> {
        let total = self.mu + self.lambda;
        let mut primer: Vec<G> = range(0, total).map(|_| self.genotype.clone()).collect();
        let mut rng = thread_rng();
        while self.current_iteration < self.iterations {
            println!("current iteration: {}", self.current_iteration);
            primer = self.iterate(&mut rng, primer.as_mut_slice());
            self.current_iteration += 1;
        }
        primer.clone()
    }

    fn iterate<R: Rng>(&self, rng: &mut R, primer: &mut [G]) -> Vec<G> {
        // shuffle the population
        shuffle(rng, primer);
        // calculate the fitness for each individual (in a separate thread)
        let (tx, rx): (Sender<(G, f64)>, Receiver<(G, f64)>) = mpsc::channel();
        for adult in primer {
            let individual = adult.clone();
            let evals = self.evaluations.clone();
            Thread::spawn(move || {
                let dungeon = individual.generate();
                let fitness = individual.evaluate(&dungeon, evals.as_slice());
                tx.send((individual, fitness)).unwrap();
            });
        }
        let mut colony: Vec<(G, f64)> = range(0, primer.len()).map(|_| rx.recv().unwrap()).collect();
        // sort by fitness
        colony.sort_by(|&a, &b| {
            let (_, f1) = a;
            let (_, f2) = b;
            match f1.partial_cmp(&f2) {
                Some(ordering) => ordering,
                None => panic!(format!("{:?} and {:?} could not be ordered.", f1, f2))
            }
        });
        // keep mu
        let mut survivors: Vec<G> = colony.iter().map(|&(i, _)| i.clone()).take(self.mu).collect();
        // add the next generation
        let next_generation: Vec<G> = range(0, self.lambda).map(|_| {
            let mut baby = self.genotype.clone();
            baby.mutate();
            baby
        }).collect();
        survivors.push_all(next_generation.as_slice());
        survivors
    }
}
