use ndarray::prelude::*;
/// Contains all information needed to define
/// a trained neural network
#[derive(Debug)]
pub struct NeuralNetwork {
    w_hidden: ArrayD<f64>,
    b_hidden: ArrayD<f64>,
    w_out: ArrayD<f64>,
    b_out: ArrayD<f64>,
}

/// Configures the neural network's attributes
// use these for neural network configuration using builder pattern [Delete struct after those builder functions are finished]
#[allow(dead_code)]
pub struct NeuralNetConfig {
    input_neurons: i64,
    output_neurons: i64,
    hidden_neurons: i64,
    num_epochs: i64,
    learning_rate: f64,
}

// setup for builder pattern [TODO]
impl NeuralNetwork {
    fn new() -> NeuralNetwork {
        NeuralNetwork {
            w_hidden: ArrayD::<f64>::zeros(IxDyn(&[0])),
            b_hidden: ArrayD::<f64>::zeros(IxDyn(&[0])),
            w_out: ArrayD::<f64>::zeros(IxDyn(&[0])),
            b_out: ArrayD::<f64>::zeros(IxDyn(&[0])),
        }
    }
}

fn main() {
    let mut net = NeuralNetwork::new();
    println!("{:?}", net);
}
