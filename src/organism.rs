// ------------------------ ORGANISM GENOME --------------------------------
#[derive(Clone, Debug)]
enum NeuronType {
    Sensor(u16),        // always a source
    Neuron(u16),        // may be a sink or source
    Action(u16),        // always a sink
}

#[derive(Clone, Debug)]
struct Gene {
    source: NeuronType,
    sink: NeuronType,
    weight: f32,
    bias: Option<f32>,
    // does not specify the activation function for now but may do so in future
    // activation_function: fn(f32) -> (),
}

impl Gene {
    fn random() -> Self {
        Self {
            source: NeuronType::Sensor(0),
            sink: NeuronType::Sensor(0),
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
struct Brain;

impl Brain {
    pub fn from_genome(_genome: &Genome) -> Self {
        Self
    }
}

// ------------------------------- ORGANISM  -------------------------------
#[derive(Clone, Debug)]
pub struct Organism {
    genome: Genome,
    brain: Brain,
}

impl Default for Organism {
    fn default() -> Self {
        let genome = vec![Gene::random()];
        let brain = Brain::from_genome(&genome);
        Self {
            genome,
            brain,
        }
    }
}
