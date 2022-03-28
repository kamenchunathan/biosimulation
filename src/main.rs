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


    let params = SimParams::from_ini(config);
    Simulator::run(params);

    Ok(())
}
