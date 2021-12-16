use rand;
use rand::{thread_rng, Rng};

use rand_derive2::RandGen;


// TODO: Move these constants to a config option
const GENOME_INITIAL_MIN_SIZE: u32 = 2;
const GENOME_INITIAL_MAX_SIZE: u32 = 20;
const GENOME_INITIAL_MIN_CONNECTIONS: u32 = 1;
const GENOME_INITIAL_MAX_CONNECTIONS: u32 = 20;

// ------------------------ ORGANISM GENOME --------------------------------
#[derive(Clone, Debug, RandGen)]
enum SensorType {
    LocationX,
    LocationY,
    Age,
    Random,
    Oscillator,
}

#[derive(Clone, Debug)]
struct SensorData {
    id: u32,
    sensor_type: SensorType,
}


#[derive(Clone, Debug)]
struct InterNeuronData {
    id: u32,
    bias: f64,
}

#[derive(Clone, Debug, RandGen)]
enum ActionType {
    MoveX,
    MoveY,
    MoveForward,
    MoveRandom,
    MoveReverse,
}

#[derive(Clone, Debug)]
struct ActionData {
    id: u32,
    action_type: ActionType,
}

#[derive(Clone, Debug)]
enum Neuron {
    Sensor(SensorData),
    InterNeuron(InterNeuronData),
    Action(ActionData),
}

#[derive(Clone, Debug)]
struct Gene {
    source: Neuron,
    sink: Neuron,
    weight: f32,
}

impl Gene {
    fn generate_random() -> Self {
        let mut rng = thread_rng();

        // only sensor or interneuron can be source
        let source = if rand::random() {
            Neuron::Sensor(
                SensorData {
                    id: rng.gen_range(GENOME_INITIAL_MIN_SIZE..GENOME_INITIAL_MAX_SIZE),
                    sensor_type: SensorType::generate_random(),
                }
            )
        } else {
            Neuron::InterNeuron(
                InterNeuronData {
                    id: rng.gen_range(GENOME_INITIAL_MIN_SIZE..GENOME_INITIAL_MAX_SIZE),
                    bias: rng.gen(),
                }
            )
        };

        // only interneuron or action neuron can be sink
        let sink = if rand::random() {
            Neuron::Action(
                ActionData {
                    id: rng.gen_range(GENOME_INITIAL_MIN_SIZE..GENOME_INITIAL_MAX_SIZE),
                    action_type: ActionType::generate_random(),
                }
            )
        } else {
            Neuron::InterNeuron(
                InterNeuronData {
                    id: rng.gen_range(GENOME_INITIAL_MIN_SIZE..GENOME_INITIAL_MAX_SIZE),
                    bias: rng.gen(),
                }
            )
        };

        let weight = rng.gen();

        Self {
            source,
            sink,
            weight,
        }
    }
}

type Genome = Vec<Gene>;

// ------------------------ ORGANISM BRAIN ---------------------------------
//  A neural network representing how the organism is able to make decisions on where to go
#[derive(Clone, Debug)]
struct NeuralNetwork;

impl NeuralNetwork {
    pub fn from_genome(_genome: &Genome) -> Self {
        Self
    }
}

// ------------------------------- ORGANISM  -------------------------------
#[derive(Clone, Debug)]
pub struct Agent {
    genome: Genome,
    brain: NeuralNetwork,
}

impl Default for Agent {
    fn default() -> Self {
        let mut rng = thread_rng();
        let genome_size = rng.gen_range(GENOME_INITIAL_MIN_CONNECTIONS..GENOME_INITIAL_MAX_CONNECTIONS) as u32;
        let genome = (0..genome_size).map(|_| Gene::generate_random()).collect();
        println!("{:?}", genome);

        let brain = NeuralNetwork::from_genome(&genome);

        Self {
            genome,
            brain,
        }
    }
}
