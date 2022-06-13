use image::ColorType;
use image::png::PNGEncoder;
use num::Complex;
use std::fs::File;
use std::io::Error;
use std::str::FromStr;
use std::env;

#[derive(Copy, Clone)]
struct Point<T> {
    x: T, y: T
}


fn write_image(filename: &str, pixels: &[u8], bounds: Point<usize>)
    -> Result<(), Error>
{
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels,
                    bounds.x as u32, bounds.y as u32,
                    ColorType::Gray(8))?;
    Ok(())
}

fn render(pixels: &mut [u8],
          bounds: Point<usize>,
          upper_left: Complex<f64>,
          lower_right: Complex<f64>)
{
    assert!(pixels.len() == bounds.x * bounds.y);

    for row in 0..bounds.y {
        for column in 0..bounds.x {
            let point = pixel_to_point(bounds, Point {x: column, y: row},
                                       upper_left, lower_right);
            pixels[row * bounds.x + column] =
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
                };
        }
    }
}


fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 9.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

#[test]
fn test_escape_time() {
    assert_eq!(escape_time(Complex {re: 0.0, im: 0.0}, 255), None); 
    assert_eq!(escape_time(Complex {re: 0.0, im: 2.0}, 255), Some(2)); 
}

fn pixel_to_point(bounds: Point<usize>,
                  pixel: Point<usize>,
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>)
    -> Complex<f64>
{
    let (width, height) = (lower_right.re - upper_left.re,
                           upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.x as f64 * width / bounds.x as f64,
        im: upper_left.im - pixel.y as f64 * height / bounds.y as f64
    }
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => (
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        )
    } 
} 

fn parse_complex<T: FromStr>(num: &str) -> Option<Complex<T>> {
    parse_pair::<T>(num, ',').map(|(re, im)| Complex { re, im })
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex::<usize>("1,2"), Some(Complex {re: 1, im: 2}));
    assert_eq!(parse_complex::<f32>("1,2"), Some(Complex {re: 1.0, im: 2.0}));
}

fn parallel_render(pixels: &mut [u8],
                   bounds: Point<usize>,
                   upper_left: Complex<f64>,
                   lower_right: Complex<f64>)
{
    let threads = 8;
    let rows_per_band = bounds.y / threads + 1;

    let bands: Vec<&mut [u8]> =
        pixels.chunks_mut(rows_per_band * bounds.x).collect();
    crossbeam::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.x;
            let band_bounds = Point { x: bounds.x, y: height};
            let band_upper_left =
                pixel_to_point(bounds, Point{x: 0, y: top}, upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, Point{x: bounds.x, y: top + height},
                               upper_left, lower_right);

            spawner.spawn(move |_| {
                render(band, band_bounds, band_upper_left, band_lower_right);
            });
        }
    }).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} file pixels upperleft lowerright", args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').map(|(x,y)| Point {x,y})
        .expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3])
        .expect("error parsing upper left");
    let lower_right = parse_complex(&args[4])
        .expect("error parsing lower right");

    let mut pixels = vec![0; bounds.x * bounds.y];

    parallel_render(&mut pixels, bounds, upper_left, lower_right);

    write_image(&args[1], &pixels, bounds)
        .expect("error writing PNG file");

}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10,20)));
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point(Point {x: 100, y: 200}, Point {x: 25, y: 175},
                              Complex { re: -1.0, im:  1.0 },
                              Complex { re:  1.0, im: -1.0 }),
               Complex { re: -0.5, im: -0.75 });
}
