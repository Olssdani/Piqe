use std::{
    f32::consts::{E, PI},
    fmt,
};

pub struct Gaussian {
    kernel: Vec<Vec<f32>>,
    size: i32,
}

impl Gaussian {
    pub fn new(size: i32, sigma: f32) -> Gaussian {
        let mut res = vec![vec![0.0; (size * 2 + 1) as usize]; (size * 2 + 1) as usize];

        let mut sum = 0.0;

        let c = 1.0 / (2.0 * PI * sigma.powi(2));

        for y in -size..=size {
            for x in -size..=size {
                let n = (x.pow(2) + y.pow(2)) as f32;
                let t = 2.0 * sigma.powi(2);

                res[(y + size) as usize][(x + size) as usize] = c * E.powf(-n / t);
                sum += res[(y + size) as usize][(x + size) as usize];
            }
        }

        for y in &mut res {
            for x in y {
                *x /= sum;
            }
        }

        Gaussian { kernel: res, size }
    }

    pub fn value(&self, x: i32, y: i32) -> f32 {
        if x.abs() > self.size || y.abs() > self.size {
            panic!(
                "Out of bound size is between -{} - {}! Your value is x: {} y: {}",
                self.size, self.size, x, y
            );
        }

        self.kernel[(y + self.size) as usize][(x + self.size) as usize]
    }

    pub fn print_normalized(&self) {
        let mult = 1.0 / self.kernel[1][1];

        for y in 0..(self.size * 2 + 1) {
            for x in 0..(self.size * 2 + 1) {
                print!(
                    "{} ",
                    (self.kernel[y as usize][x as usize] * mult).round() as i32
                );
            }
            println!();
        }
    }
}

impl fmt::Display for Gaussian {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..(self.size * 2 + 1) {
            for x in 0..(self.size * 2 + 1) {
                write!(f, "{} ", self.kernel[y as usize][x as usize]).expect("wrong");
            }
            writeln!(f).expect("wrong");
        }
        Ok(())
    }
}
