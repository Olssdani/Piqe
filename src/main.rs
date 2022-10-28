use image::io::Reader;
use std::{
    f32::consts::{E, PI},
    fmt,
    io::Cursor,
    thread::panicking,
};

fn mscn(i: f32, u: f32, sigma: f32) -> f32 {
    // MSCN(x) = (I(x) - u(x)) / (sigma(x) +  Epislon)
    //Epsilon is a small number to make sure we do not make division by 0
    (i - u) / (sigma + 0.0039)
}

struct Gaussian {
    kernel: Vec<Vec<f32>>,
    size: i32,
}

impl Gaussian {
    fn new(size: i32, sigma: f32) -> Gaussian {
        let mut res = vec![vec![0.0; (size * 2 + 1) as usize]; (size * 2 + 1) as usize];
        for y in -size..=size {
            for x in -size..=size {
                res[(y + size) as usize][(x + size) as usize] = (1.0 / (2.0 * PI * sigma.powi(2)))
                    * E.powf(-((x.pow(2) + y.pow(2)) as f32 / (2.0 * sigma.powi(2))))
            }
        }

        Gaussian { kernel: res, size }
    }

    fn value(&self, x: i32, y: i32) -> f32 {
        if x.abs() > self.size || y.abs() > self.size {
            panic!(
                "Out of bound size is between -{} - {}! Your value is x: {} y: {}",
                self.size, self.size, x, y
            );
        }

        self.kernel[(y + self.size) as usize][(x + self.size) as usize]
    }

    fn size(&self) -> i32 {
        self.size
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

fn main() {
    let size = 3;
    let kernel = Gaussian::new(size, 1.0);
    println!("{}", kernel);

    // Read Image, What is the format? Intesity image == grey scale?
    let img = Reader::open("test.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_luma32f();

    let width = img.width() as usize;
    let height = img.height() as usize;

    let mut u = vec![vec![0.0; width]; height];

    for y in 0..height {
        for x in 0..width {
            for dy in -size..=size {
                if y as i32 + dy < 0 || y as i32 + dy >= height as i32 {
                    continue;
                }

                for dx in -size..=size {
                    if x as i32 + dx < 0 || x as i32 + dx >= width as i32 {
                        continue;
                    }
                    let kx = x as i32 + dx;
                    let ky = y as i32 + dy;

                    let pixel = img.get_pixel(kx as u32, ky as u32);
                    u[y][x] += kernel.value(dx, dy) * pixel[0];
                }
            }
        }
    }

    let mut sigma = vec![vec![0.0; width]; height];

    for y in 0..height {
        for x in 0..width {
            for dy in -size..=size {
                if y as i32 + dy < 0 || y as i32 + dy >= height as i32 {
                    continue;
                }

                for dx in -size..=size {
                    if x as i32 + dx < 0 || x as i32 + dx >= width as i32 {
                        continue;
                    }
                    let kx = x as i32 + dx;
                    let ky = y as i32 + dy;

                    let pixel = img.get_pixel(kx as u32, ky as u32);
                    sigma[y][x] += (kernel.value(dx, dy)
                        * (pixel[0] - u[ky as usize][kx as usize]).powi(2))
                    .sqrt();
                }
            }
        }
    }

    let mut mscn_values = vec![vec![0.0; width]; height];

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x as u32, y as u32);
            mscn_values[y][x] = mscn(pixel[0], u[y][x], sigma[y][x]);
        }
    }

    let bx = width / 16;
    let by = height / 16;
    println!("bx {}, by {}", bx, by);

    let mut blocks = vec![vec![0.0; bx]; by];

    for y in 0..by {
        for x in 0..bx {
            let mut sum = 0.0;
            for dy in 0..16 {
                for dx in 0..16 {
                    sum += mscn_values[y * 16 + dy][x * 16 + dx];
                }
            }
            let mean = sum / (16.0 * 16.0);

            let mut variance = 0.0;
            for dy in 0..16 {
                for dx in 0..16 {
                    variance += (mscn_values[y * 16 + dy][x * 16 + dx] - mean).powi(2);
                }
            }

            blocks[y][x] = variance / (16.0 * 16.0)
        }
    }

    for y in 0..by {
        for x in 0..bx {
            println!("{}", blocks[y][x]);
        }
    }
}
