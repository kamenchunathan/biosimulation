// Organism brain which is a neural network

#[derive(Clone, Copy, Debug)]
struct Brain;

impl Brain {
    pub fn random() -> Self {
        // TODO: create a random brain
        //  really a stub function that's never needed as brain is always created from a gene
        Self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Organism {
    gene: u8,
    brain: Brain,
}

impl Default for Organism {
    fn default() -> Self {
        Self {
            gene: 0,
            brain: Brain::random(),
        }
    }
}
