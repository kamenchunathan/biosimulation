use configparser::ini::Ini;
use rayon::prelude::*;

use crate::organism::Agent;

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

        let mut population: Vec<_> = (0..(params.world_size.0 * params.world_size.1))
            .map(|_| Agent::gen_with_random_genes())
            .collect();

        // Iterate over all organisms in the world
        // Main loop
        for _sim in 0..params.num_sims {
            for _sim_step in 0..params.steps_per_sim {
                let new_world = population.into_par_iter().map(|agent| agent).collect();
                // TODO: Do some processing e.g. tracking states
                population = new_world;
            }
        }
    }
}

impl SimParams {
    pub fn from_ini(config: Ini) -> SimParams {
        let world_size = (
            config.getint("world", "height").unwrap().unwrap() as usize,
            config.getint("world", "width").unwrap().unwrap() as usize
        );
        let num_sims = config.getint("simulation", "num_simulations").unwrap().unwrap() as usize;
        let steps_per_sim = config.getint("simulation", "steps_per_sim").unwrap().unwrap() as usize;

        SimParams {
            world_size,
            num_sims,
            steps_per_sim,
        }
    }
}