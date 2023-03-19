// Question asked of ChatGPT:
// write a rust program that generates a fractal 2D image where the coordinate of each pixel is on the complex plane and the color of the pixel indicates how many iterations of Newton's method is required to get to within distance 0.0000001 of one of the three complex roots of y = x**3 + 1, and each root uses a different color.

// Answer from ChatGPT:
// Here's an example program that generates a fractal image using Newton's method on the complex plane:

use image::{ImageBuffer, Rgb};
use num_complex::Complex;

fn newton_iteration(c: Complex<f64>) -> (u8, Complex<f64>) {
    let mut z = c;
    let mut i = 255;
    let order = 8;
    while i > 0 {
        let delta = (z.powi(order) - 1.0) / (order as f64 * z.powi(order - 1));
        if delta.norm() < 1e-7 {
            break;
        }
        z = z - delta;
        i = i - 1;
        // i = if i > 32 { i - 4 } else { i - 1 };
    }
    (i, z)
}

fn main() {
    let _roots = vec![
        Complex { re: -1.0, im: 0.0 },
        Complex { re: 0.5, im: 0.5 * 3.0f64.sqrt() },
        Complex { re: 0.5, im: -0.5 * 3.0f64.sqrt() },
    ];
    let (width, height) = (1920*8, 1080*8);
    let mut img = ImageBuffer::new(width, height);

    let zoom: f64 = 0.2;
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let c = Complex {
            re: zoom * (16.0 * (x as f64) / (width as f64) - 8.0),
            im: zoom * (9.0 * (y as f64) / (height as f64) - 4.5),
        };
        let (i, z) = newton_iteration(c);
        *pixel = if i == 0 {
            Rgb::<u8>([0,0,0])
        } else { 
            match (sign(z.re), sign(z.im)) {
                (1, 1) => Rgb::<u8>([i, 0, 0]), // Red
                (1, 0) => Rgb::<u8>([i, i/2, 0]),
                (1, -1) => Rgb::<u8>([i, i, 0]), // yellow
                (0, -1) => Rgb::<u8>([0, i, 0]), // green
                (-1, -1) => Rgb::<u8>([0, i, i/3*2]),
                (-1, 0) => Rgb::<u8>([0, i/3*2, i]),
                (-1, 1) => Rgb::<u8>([i/4, i/4, i]), // blue
                (0, 1) => Rgb::<u8>([i, 0, i]),
                (_, _) => Rgb::<u8>([0, 0, 0]),
            }
        }
    }

    img.save("fractal.png").unwrap();
}

fn sign(x: f64) -> isize {
    if x > 1e-7 { 1 } else if x < -1e-7 { -1 } else { 0 }
}


// This program uses the image crate to save the fractal image as a PNG file. It also uses the num-complex crate to represent complex numbers. You can install these crates by adding the following dependencies to your Cargo.toml file:

// [dependencies]
// image = "0.23.5"
// num-complex = "0.3.0"
  
// Please note that the above code is just one way to implement the fractal, different parameter values and different calculations can lead to different images.
