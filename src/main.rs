fn main() {
    println!("Hello, world!");
}

pub struct Network {
    layers: Vec<Layer>,
}
struct Layer {
    neurons: Vec<Neuron>,
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        let output = input
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>()

        (output + self.bias).max(0.0)
    }
}

pub fn random(layers: Vec<LayerTopology>) -> Self {
    assert!(layers.len() > 1);

    let layers = layer
        .windows(2)
        .map(|layers| {
            Layer::random(layers[0].neurons, layers[1].neurons)
        })
        .collect();

    Self { layers }
}
