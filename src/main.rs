use gaussian_kernel::Gaussian;
use image::io::Reader;
use mscn::MSCN;
use std::f32::EPSILON;

mod gaussian_kernel;
mod mscn;

fn mscn(i: f32, u: f32, sigma: f32) -> f32 {
    // MSCN(x) = (I(x) - u(x)) / (sigma(x) +  Epislon)
    //Epsilon is a small number to make sure we do not make division by 0
    (i - u) / (sigma + EPSILON)
}

fn main() {
    let size = 3;
    let kernel = Gaussian::new(size, 1.0);

    println!("{}", kernel);
    kernel.print_normalized();

    // Read Image, What is the format? Intesity image == grey scale?
    let img = Reader::open("test.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_luma32f();

    let width = img.width() as usize;
    let height = img.height() as usize;

    let mscn = MSCN::new();

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

    /* for y in 0..by {
        for x in 0..bx {
            println!("{}", blocks[y][x]);
        }
    }*/
}
