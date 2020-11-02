#[allow(dead_code)]
#[allow(unused_variables)]

pub mod nn {
    use crate::matrix::matrix::Matrix;
    use std::f64::consts::E;

    #[derive(Debug, Clone, Copy)]
    pub struct ActivationFunction {
        pub func: fn(f32) -> f32,
        pub dfunc: fn(f32) -> f32,
    }

    const SIGMOID: ActivationFunction = ActivationFunction {
        func: |x| 1.0 / (1.0 + E.powf(x.into())) as f32,
        dfunc: |y| y * (1.0 - y),
    };
    const TANH: ActivationFunction = ActivationFunction {
        func: |x| x.tanh(),
        dfunc: |y| 1.0 - (y * y),
    };

    #[derive(Debug, Clone)]
    pub struct NeuralNetwork {
        pub input_nodes: i32,
        pub hidden_nodes: i32,
        pub output_nodes: i32,
        pub weights_ih: Matrix,
        pub weights_ho: Matrix,
        pub bias_h: Matrix,
        pub bias_o: Matrix,
        learning_rate: f32,
        activation_function: ActivationFunction,
    }

    impl NeuralNetwork {
        pub fn new(in_nodes: i32, hid_nodes: i32, out_nodes: i32) -> NeuralNetwork {
            NeuralNetwork {
                input_nodes: in_nodes,
                hidden_nodes: hid_nodes,
                output_nodes: out_nodes,

                weights_ih: Matrix::new(hid_nodes, in_nodes).randomize(),
                weights_ho: Matrix::new(out_nodes, hid_nodes).randomize(),

                bias_h: Matrix::new(hid_nodes, 1).randomize(),
                bias_o: Matrix::new(out_nodes, 1).randomize(),

                learning_rate: 0.1,
                activation_function: SIGMOID,
            }
        }

        pub fn from_nn(nn: &NeuralNetwork) -> NeuralNetwork {
            NeuralNetwork {
                input_nodes: nn.input_nodes,
                hidden_nodes: nn.hidden_nodes,
                output_nodes: nn.output_nodes,

                weights_ih: nn.weights_ih.clone(),
                weights_ho: nn.weights_ho.clone(),

                bias_h: nn.bias_h.clone(),
                bias_o: nn.bias_o.clone(),

                learning_rate: 0.1,
                activation_function: SIGMOID,
            }
        }

        pub fn copy(&self) -> NeuralNetwork {
            NeuralNetwork::from_nn(self)
        }

        pub fn set_learning_rate(&mut self, learning_rate: f32) {
            self.learning_rate = learning_rate;
        }

        pub fn set_activation_function(&mut self, func: ActivationFunction) {
            self.activation_function = func;
        }

        pub fn predict(&self, input_array: &[f32]) -> Result<Vec<f32>, String> {
            // Generating the Hidden Outputs
            let inputs = Matrix::from_array(input_array);
            let mut hidden = self.weights_ih.cross_product(&inputs)?;
            hidden = hidden.add_m(&self.bias_h)?;
            hidden = hidden.map(|x, i, j| (self.activation_function.func)(x));
            // Generating the output's output!
            let mut output = self.weights_ho.cross_product(&hidden)?;
            output = output.add_m(&self.bias_o)?;
            output = output.map(|x, i, j| (self.activation_function.func)(x));
            Ok(output.to_array())
        }

        pub fn mutate<F>(&mut self, func: F)
        where
            F: Fn(f32) -> f32,
        {
            self.weights_ih = self.weights_ih.map(|x, i, j| func(x));
            self.weights_ho = self.weights_ho.map(|x, i, j| func(x));
            self.bias_h = self.bias_h.map(|x, i, j| func(x));
            self.bias_o = self.bias_o.map(|x, i, j| func(x));
        }
    }
}
