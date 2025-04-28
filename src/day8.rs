use core::str;

use itertools::Itertools;

pub fn solve_part_one(test: bool) {
    let image = load_data(test);

    let mut layer_number = 0;
    let mut minimum_zero_count = usize::MAX;
    for (number, layer) in image.layers.iter().enumerate() {
        let zero_count = count(0, layer);
        if zero_count < minimum_zero_count {
            minimum_zero_count = zero_count;
            layer_number = number;
        }
    }

    let one_count = count(1, image.layer(layer_number));
    let two_count = count(2, image.layer(layer_number));
    println!("Part 1: {}", one_count * two_count);
}

pub fn solve_part_two(test: bool) {
    let image = load_data(test);
    let decoded_image = image.decode();

    println!("Part 2:");
    decoded_image[..].chunks(image.width).for_each(|line| {
        println!(
            "{}",
            line.iter()
                .map(|d| match d {
                    1 => "*",
                    _ => " ",
                })
                .join("")
        )
    });
}

struct Image {
    layers: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Image {
    fn layer(&self, layer: usize) -> &Vec<u32> {
        &self.layers[layer]
    }

    fn decode(&self) -> Vec<u32> {
        (0..self.width * self.height)
            .map(|pixel| {
                let mut layer = 0;
                while layer < self.layers.len() && self.layers[layer][pixel] == 2 {
                    layer += 1;
                }
                self.layers[layer][pixel]
            })
            .collect()
    }
}

fn count(value: u32, layer: &[u32]) -> usize {
    layer.iter().filter(|pixel| **pixel == value).count()
}

fn load_data(test: bool) -> Image {
    let width = if test { 3 } else { 25 };
    let height = if test { 2 } else { 6 };

    let layers = aoc2019::load_data(8, test)
        .trim()
        .as_bytes()
        .chunks(width * height)
        .map(|slice| str::from_utf8(slice).expect("Oops..").to_owned())
        .map(|layer| layer.chars().map(|d| d.to_digit(10).unwrap()).collect())
        .collect();

    Image {
        layers,
        width,
        height,
    }
}
