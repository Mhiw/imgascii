use std::{
    env,
    process::exit,
};
use image::GenericImageView;

fn map(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    let clamped_input = value.max(in_min).min(in_max);

    let input_range = in_max - in_min;
    let input_percent = (clamped_input - in_min) / input_range;

    let output_range = out_max - out_min;
    let output_value = out_min + (output_range * input_percent);

    output_value
}

fn main() {
    let density: String = "@%#*+=-:. ".to_string();

    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        println!("Arguments incorrect!\n\t- imgascii \"path/to/file.png\"");
        exit(1);
    }

    let image_path: &str = &arguments[1];
    let image = image::open(image_path).expect("Failed to load image!");
    let mut ascii_image: String = String::from("");

    for y in 0..image.dimensions().1 as usize {
        for x in 0..image.dimensions().0 as usize {
            let pixel = image.get_pixel(x as u32, y as u32);
            let color_average: f32 = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3 as f32;

            let character_index: usize = map(color_average, 0.0, 255.0, 0.0, density.len() as f32).round() as usize;
            let character: char = density.chars().nth(if character_index == 0 { character_index } else { character_index - 1 }).expect("Failed to get density");

            ascii_image.push(character);
        }
        ascii_image.push_str("\n");
    }

    println!("{}", ascii_image);
}
