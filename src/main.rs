use image::{ImageBuffer, RgbImage};
use rand::{distributions::Uniform, rngs::ThreadRng, Rng};
use rayon::prelude::*;
use std::sync::Mutex;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(short, long, default_value_t = 4)]
    k: usize,
}

#[derive(Debug, Clone, Copy)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Clone)]
struct Centroid {
    r: f64,
    g: f64,
    b: f64,
}

struct KMeans {
    centroids: Vec<Centroid>,
}

impl Pixel {
    fn distance(&self, centroid: &Centroid) -> f64 {
        ((self.r as f64 - centroid.r).powi(2)
            + (self.g as f64 - centroid.g).powi(2)
            + (self.b as f64 - centroid.b).powi(2))
            .sqrt()
    }
}

impl Centroid {
    fn is_close(&self, other: &Centroid) -> bool {
        let threshold = 1e-5;

        (self.r - other.r).abs() < threshold &&
        (self.g - other.g).abs() < threshold &&
        (self.b - other.b).abs() < threshold
    }
}

impl KMeans {
    fn new(k: usize, pixels: &Vec<Pixel>, rng: &mut ThreadRng) -> KMeans {
        let range = Uniform::from(0..pixels.len());
        let centroids: Vec<Centroid> = (0..k)
            .map(|_| {
                let pixel = &pixels[rng.sample(range)];
                Centroid {
                    r: pixel.r as f64,
                    g: pixel.g as f64,
                    b: pixel.b as f64,
                }
            })
            .collect();

        KMeans { centroids }
    }

    fn run(&mut self, pixels: &Vec<Pixel>, max_iterations: usize) {
        let mut assignments: Vec<usize> = vec![0; pixels.len()];

        for _ in 0..max_iterations {
            let centroids = self.centroids.clone();

            assignments.par_iter_mut().enumerate().for_each(|(i, assignment)| {
                let pixel = &pixels[i];
                let mut min_distance = f64::MAX;
                let mut closest_centroid = 0;

                for (j, centroid) in centroids.iter().enumerate() {
                    let distance = pixel.distance(centroid);

                    if distance < min_distance {
                        min_distance = distance;
                        closest_centroid = j;
                    }
                }
                *assignment = closest_centroid;
            });
            let prev_centroids = self.centroids.clone();

            self.update_centroids(&assignments, pixels);
            if self.centroids.iter().zip(prev_centroids.iter()).all(|(a, b)| a.is_close(b)) {
                break;
            }
        }
    }

    fn update_centroids(&mut self, assignments: &Vec<usize>, pixels: &Vec<Pixel>) {
        let k = self.centroids.len();
        let pixel_groups: Vec<Vec<Pixel>> = (0..k)
            .map(|_| Vec::new())
            .collect();
        let pixel_groups = pixel_groups.into_iter().map(|v| Mutex::new(v)).collect::<Vec<_>>();

        assignments.par_iter().zip(pixels.par_iter()).for_each(|(&assignment, pixel)| {
            let mut group = pixel_groups[assignment].lock().unwrap();
            group.push(*pixel);
        });

        let new_centroids: Vec<Centroid> = pixel_groups.iter().map(|group| {
            let group = group.lock().unwrap();
            let (sum_r, sum_g, sum_b, count) = group.iter().fold((0f64, 0f64, 0f64, 0), |(r, g, b, count), pixel| {
                (r + pixel.r as f64, g + pixel.g as f64, b + pixel.b as f64, count + 1)
            });

            if count > 0 {
                Centroid {
                    r: sum_r / count as f64,
                    g: sum_g / count as f64,
                    b: sum_b / count as f64,
                }
            } else {
                Centroid { r: 0.0, g: 0.0, b: 0.0 }
            }
        }).collect();

        self.centroids = new_centroids;
    }
}

fn main() {
    let args = Args::parse();
    let input = args.input;
    let output = args.output;
    let k = args.k;
    let img = image::open(input).expect("Failed to open image").to_rgb8();
    let (width, height) = img.dimensions();
    let mut pixels: Vec<Pixel> = Vec::new();

    for (_, _, pixel) in img.enumerate_pixels() {
        pixels.push(Pixel {
            r: pixel[0],
            g: pixel[1],
            b: pixel[2],
        });
    }
    let mut rng = rand::thread_rng();
    let mut kmeans = KMeans::new(k, &pixels, &mut rng);

    kmeans.run(&pixels, 100);
    let output_img = reconstruct_image(&kmeans.centroids, &pixels, width, height);

    output_img.save(output).expect("Failed to save image");
}

fn reconstruct_image(centroids: &Vec<Centroid>, pixels: &Vec<Pixel>, width: u32, height: u32) -> RgbImage {
    let mut img: RgbImage = ImageBuffer::new(width, height);

    for (i, pixel) in pixels.iter().enumerate() {
        let centroid = &centroids[closest_centroid_index(pixel, centroids)];

        img.put_pixel(
            (i as u32) % width,
            (i as u32) / width,
            image::Rgb([centroid.r as u8, centroid.g as u8, centroid.b as u8]),
        );
    }
    img
}

fn closest_centroid_index(pixel: &Pixel, centroids: &Vec<Centroid>) -> usize {
    centroids
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            pixel
                .distance(a)
                .partial_cmp(&pixel.distance(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(index, _)| index)
        .unwrap_or(0)
}
