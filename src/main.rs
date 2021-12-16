mod simulator;
mod organism;
mod visualize;

use configparser::ini::Ini;
use std::error::Error;
use crate::simulator::{SimParams, Simulator};

fn main() -> Result<(), Box<dyn Error>> {
    // config parsing
    let mut config = Ini::new();
    config.load("conf/default.ini")?;


    let params = get_sim_params(config);
    Simulator::run(params);

    Ok(())
}

fn get_sim_params(config: Ini) -> SimParams {
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