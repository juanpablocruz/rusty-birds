#[allow(dead_code)]
#[allow(unused_variables)]

pub mod matrix {
    use rand::Rng;
    #[derive(Debug, Clone)]
    pub struct Matrix {
        pub rows: i32,
        pub cols: i32,
        pub data: Vec<f32>,
    }

    impl Matrix {
        pub fn new(rows: i32, cols: i32) -> Matrix {
            Matrix {
                cols: cols,
                rows: rows,
                data: vec![0.0; (cols * rows) as usize],
            }
        }
        pub fn from_array(arr: &[f32]) -> Matrix {
            Matrix {
                rows: arr.len() as i32,
                cols: 1,
                data: arr.to_vec(),
            }
        }

        pub fn print(&self) {
            print!("\n");
            for i in 0..self.cols {
                print!("|  ");
                for j in 0..self.rows {
                    print!(" {:.2} ", self.data[((i * self.rows) + j) as usize]);
                }
                print!("  |\n");
            }
            print!("\n");
        }

        pub fn to_array(self) -> Vec<f32> {
            self.data
        }

        pub fn randomize(&self) -> Matrix {
            let mut rng = rand::thread_rng();
            let mut new_data: Vec<f32> = vec![0.0; (self.cols * self.rows) as usize];

            for i in 0..self.data.len() {
                new_data[i] = rng.gen_range(-1.0, 1.0);
            }

            let new_matrix = Matrix {
                cols: self.cols,
                rows: self.rows,
                data: new_data,
            };
            new_matrix
        }

        pub fn map<F>(&self, f: F) -> Matrix
        where
            F: Fn(f32, i32, i32) -> f32,
        {
            let mut new_data: Vec<f32> = vec![0.0; (self.cols * self.rows) as usize];
            for i in 0..self.cols {
                for j in 0..self.rows {
                    let index = ((i * self.rows) + j) as usize;
                    new_data[index] = f(self.data[index], i, j);
                }
            }
            Matrix {
                rows: self.rows,
                cols: self.cols,
                data: new_data,
            }
        }

        pub fn transpose(&self) -> Matrix {
            let mut new_data = vec![0.0; (self.cols * self.rows) as usize];
            for i in 0..self.cols {
                for j in 0..self.rows {
                    new_data[((i * self.rows) + j) as usize] =
                        self.data[((j * self.cols) + i) as usize];
                }
            }
            Matrix {
                rows: self.rows,
                cols: self.cols,
                data: new_data,
            }
        }

        pub fn mul(&self, _rhs: f32) -> Matrix {
            let new_data = self.data.iter().map(|e| e * _rhs).collect();
            Matrix {
                cols: self.cols,
                rows: self.rows,
                data: new_data,
            }
        }
        pub fn cross_product(&self, _rhs: &Matrix) -> Result<Matrix, String> {
            if self.cols != _rhs.rows {
                return Err("Columns of A must match rows of B.".to_string());
            }
            let mut new_data = vec![0.0; (self.rows * _rhs.cols) as usize];
            for i in 0..self.rows {
                for j in 0..self.cols {
                    let v = self.data[((i * self.cols) + j) as usize];
                    let mut acc = 0.0;
                    for k in 0.._rhs.rows {
                        acc += v * _rhs.data[k as usize];
                    }
                    new_data[i as usize] = acc;
                }
            }
            let new_matrix = Matrix {
                cols: _rhs.cols,
                rows: self.rows,
                data: new_data,
            };
            Ok(new_matrix)
        }

        fn add(&self, _rhs: f32) -> Matrix {
            Matrix {
                cols: self.cols,
                rows: self.rows,
                data: self.data.iter().map(|x| x + _rhs).collect(),
            }
        }

        pub fn add_m(&self, _rhs: &Matrix) -> Result<Matrix, String> {
            if self.rows != _rhs.rows || self.cols != _rhs.cols {
                return Err("Columns and Rows of A must match Columns and Rows of B".to_string());
            }
            let mut new_data = vec![0.0; (self.cols * _rhs.rows) as usize];
            for (i, (aval, bval)) in self.data.iter().zip(&_rhs.data).enumerate() {
                new_data[i] = aval + bval;
            }
            Ok(Matrix {
                cols: self.cols,
                rows: self.rows,
                data: new_data,
            })
        }

        pub fn sub_m(&self, _rhs: &Matrix) -> Result<Matrix, String> {
            if self.rows != _rhs.rows || self.cols != _rhs.cols {
                return Err("Columns and Rows of A must match Columns and Rows of B".to_string());
            }
            let mut new_data = vec![0.0; (self.cols * _rhs.rows) as usize];
            for (i, (aval, bval)) in self.data.iter().zip(&_rhs.data).enumerate() {
                new_data[i] = aval - bval;
            }
            Ok(Matrix {
                cols: self.cols,
                rows: self.rows,
                data: new_data,
            })
        }
        pub fn sub(&self, _rhs: f32) -> Matrix {
            Matrix {
                cols: self.cols,
                rows: self.rows,
                data: self.data.iter().map(|x| x - _rhs).collect(),
            }
        }
    }
}
