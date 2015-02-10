use rand::{Rng, thread_rng};

use evaluation::{EvaluationFn};
use genotype::{GenoType};
use util::{shuffle};

pub struct MuLambda<'a, G: GenoType> {
    iterations: usize,
    current_iteration: usize,
    mu: usize,     // number to keep
    lambda: usize, // number to generate
    genotype: G,
    evaluations: &'a [EvaluationFn]
}

impl<'a, G: GenoType + Clone> MuLambda<'a, G> {
    pub fn new(iterations: usize, mu: usize, lambda: usize, genotype: G, funcs: &'a [EvaluationFn]) -> MuLambda<'a, G> {
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
            primer = self.iterate(&mut rng, primer.as_mut_slice());
            self.current_iteration += 1;
        }
        primer.clone()
    }

    fn iterate<R: Rng>(&self, rng: &mut R, primer: &mut [G]) -> Vec<G> {
        // shuffle the population
        shuffle(rng, primer);
        // calculate the fitness for each individual
        let mut colony: Vec<(&G, f64)> = primer.iter().map(|individual| {
            let dungeon = individual.generate();
            let fitness = individual.evaluate(&dungeon, self.evaluations);
            (individual, fitness)
        }).collect();
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
