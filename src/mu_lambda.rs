
pub struct MuLambda {
    iterations: usize,
    mu: usize,     // number to keep
    lambda: usize  // number to generate
}

impl MuLambda {
    pub fn new(iterations: usize, mu: usize, lambda: usize) -> MuLambda {
        MuLambda {
            iterations: iterations,
            mu: mu,
            lambda: lambda
        }
    }
}
