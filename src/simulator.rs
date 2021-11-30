use rayon::prelude::*;
use crate::organism::Organism;

pub struct Simulator;

pub struct SimParams {
    pub(crate) world_size: (usize, usize),
    pub(crate) num_sims: usize,
    pub(crate) steps_per_sim: usize,
}


impl Simulator {
    pub fn run(params: SimParams) {
        // TODO: Print debug info
        // Create a world

        let mut world = vec![Organism::default(); params.world_size.0 * params.world_size.1];

        // Iterate over all organisms in the world
        // Main loop
        for _sim in 0..params.num_sims {
            for _sim_step in 0..params.steps_per_sim {
                let new_world = world.into_par_iter().map(|organism| organism).collect();
                // TODO: Do some processing e.g. tracking states
                world = new_world;
            }
        }
    }
}