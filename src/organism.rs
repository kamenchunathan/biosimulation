use std::collections::{HashMap, HashSet};
use std::fmt;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use rand;
use rand::{thread_rng, Rng};

use rand_derive2::RandGen;
use crate::visualize::visualize_nnet;

// TODO:
//              --------->>  START HERE NEXT TIME <<---------
//  Major problem how im inserting neurons into the genome and handling conversion of gene to
//  Neural network causes some sensor neurons to have incoming connections and Action Neurons
//  to have outgoing connections which is not ideal
//  .
//  1. Extract Neuron and gene structs into separate module
//  2. Create multiple alternative ways of initializing a genome e.g. full random chance, from
//   stub/ template
//  3. Create trackers the statistical characteristics of the genes e.g. level of
//   connectedness, avg. num of nodes between sensors and action nodes, & other graph xtics
//   and see which of these characteristics provided by the different algorithms provide the best
//   chances of success in the solution space
//  4. Create tests to ensure gene is initialized as intended e.g. contains no action neurons
//   with outputs. Or look into alternative ways of handling defective genes for example being
//   very lenient and trying to create a most probable working nnet or just pause and stop there


// TODO: Move these constants to a config option
const GENOME_INITIAL_MIN_SIZE: u32 = 2;
const GENOME_INITIAL_MAX_SIZE: u32 = 20;
const GENOME_INITIAL_MIN_CONNECTIONS: u32 = 1;
const GENOME_INITIAL_MAX_CONNECTIONS: u32 = 10;

// ------------------------ ORGANISM GENOME --------------------------------
#[derive(Clone, Debug, RandGen)]
pub(crate) enum SensorType {
    LocationX,
    LocationY,
    Age,
    Random,
    Oscillator,
}

#[derive(Clone, Debug, RandGen)]
pub(crate) enum ActionType {
    MoveX,
    MoveY,
    MoveForward,
    MoveRandom,
    MoveReverse,
}

#[derive(Clone, Debug)]
pub(crate) enum NeuronType {
    Sensor(SensorType),
    InterNeuron(f32),
    Action(ActionType),
}

impl NeuronType {
    fn get_output(&self) -> f32 {
        // TODO: Complete implementation
        match self {
            NeuronType::Sensor(sensor_type) => 0.0,
            NeuronType::InterNeuron(bias) => 0.0,
            NeuronType::Action(action_type) => 0.0,
        }
    }
}

// required for changing into a string used by graphviz to display string
impl fmt::Display for NeuronType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
struct Neuron {
    id: u32,
    neuron_type: NeuronType,
}

// A connection between 2 neurons
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
            Neuron {
                id: rng.gen_range(GENOME_INITIAL_MIN_SIZE..GENOME_INITIAL_MAX_SIZE),
                neuron_type: NeuronType::Sensor(SensorType::generate_random()),
            }
        } else {
            Neuron {
                id: rng.gen_range(GENOME_INITIAL_MIN_SIZE..GENOME_INITIAL_MAX_SIZE),
                neuron_type: NeuronType::InterNeuron(rng.gen()),
            }
        };

        // only interneuron or action neuron can be sink
        let sink = if rand::random() {
            Neuron {
                id: rng.gen_range(GENOME_INITIAL_MIN_SIZE..GENOME_INITIAL_MAX_SIZE),
                neuron_type: NeuronType::Action(ActionType::generate_random()),
            }
        } else {
            Neuron {
                id: rng.gen_range(GENOME_INITIAL_MIN_SIZE..GENOME_INITIAL_MAX_SIZE),
                neuron_type: NeuronType::InterNeuron(rng.gen()),
            }
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
pub(crate) struct NeuralNetwork(pub(crate) Graph<NeuronType, f32>);

impl NeuralNetwork {
    fn from_genome(genome: &Genome) -> Self {
        let mut nnet_graph = Graph::<NeuronType, f32>::new();
        let mut node_index_lookup = HashMap::<u32, NodeIndex>::new();

        // add nodes
        let neuron_list = get_neuron_list(genome);
        for neuron in neuron_list {
            node_index_lookup.insert(neuron.id, nnet_graph.add_node(neuron.neuron_type.clone()));
        }

        // connections
        for gene in genome {
            nnet_graph.add_edge(
                *node_index_lookup.get(&gene.source.id).unwrap(),
                *node_index_lookup.get(&gene.sink.id).unwrap(),
                gene.weight
            );
        }

        Self(nnet_graph)
    }
}


// helper methods in converting genome to neural network
fn get_neuron_list(genome: &Genome) -> Vec<&Neuron> {
    let mut uniq = Vec::<&Neuron>::new();
    let mut ids = HashSet::<u32>::new();

    for gene in genome {
        [&gene.source, &gene.sink].map(|neuron| {
            if !ids.contains(&neuron.id) {
                ids.insert((neuron.id));
                uniq.push(neuron);
            }
        });
    }

    uniq
}


// ------------------------------- ORGANISM  -------------------------------
#[derive(Clone, Debug)]
pub struct Agent {
    genome: Genome,
    brain: NeuralNetwork,
}

impl Agent {
    pub(crate) fn gen_with_random_genes() -> Self {
        let mut rng = thread_rng();
        let genome_size = rng.gen_range(GENOME_INITIAL_MIN_CONNECTIONS..GENOME_INITIAL_MAX_CONNECTIONS) as usize;
        let genome = (0..genome_size).map(|_| Gene::generate_random()).collect();

        let brain = NeuralNetwork::from_genome(&genome);

        visualize_nnet(&brain);

        Self {
            genome,
            brain,
        }
    }
}
