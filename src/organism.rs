// ------------------------ ORGANISM GENOME --------------------------------
#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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
    fn random() -> Self {
        Self {
            source: Neuron::Sensor(
                SensorData {
                    id: 0,
                    sensor_type: SensorType::Random
                }
            ),
            sink: Neuron::Action(
                ActionData{
                    id: 1,
                    action_type: ActionType::MoveRandom
                }
            ),
            weight: 0.0,
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
        let genome = vec![Gene::random()];
        let brain = NeuralNetwork::from_genome(&genome);

        Self {
            genome,
            brain,
        }
    }
}
