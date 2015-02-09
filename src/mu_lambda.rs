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

    pub fn evaluate(&mut self) {
        let total = self.mu + self.lambda;
        let mut primer: Vec<G> = range(0, total).map(|_| self.genotype.clone()).collect();
        let mut rng = thread_rng();

        while self.current_iteration < self.iterations {
            // TODO: implement fitness functions first
            self.iterate(&mut rng, primer.as_mut_slice());
            self.current_iteration += 1;
        }

    }

    fn iterate<R: Rng>(&self, rng: &mut R, primer: &mut [G]) {
        shuffle(rng, primer);
        for individual in primer.iter() {
            let dungeon = individual.generate();
            // individual.evaluate(dungeon, fitness_tests)
        }

        // sort by fitness
        // keep mu
        // add the next generation
    }
}
