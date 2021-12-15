// ------------------------ ORGANISM GENOME --------------------------------
#[derive(Clone, Debug)]
enum Neuron {
    Sensor,             // always a source
    Neuron(u16),        // may be a sink or source
    Action(u16),        // always a sink
}

#[derive(Clone, Debug)]
struct Gene {
    source: Neuron,
    sink: Neuron,
    weight: f32,
    bias: Option<f32>,
    // does not specify the activation function for now but may do so in future
    // activation_function: fn(f32) -> (),
}

impl Gene {
    fn random() -> Self {
        Self {
            source: Neuron::Sensor,
            sink: Neuron::Sensor,
            weight: 0.0,
            bias: None,
            // activation_function: |a|{},
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
