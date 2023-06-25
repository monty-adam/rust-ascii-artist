//! An example of opening an image.
extern crate image;

use std::{env, path::Path};
use image::{GenericImageView, Pixel};

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let im = image::open(&Path::new(&file)).unwrap();

    for (x, y, color) in im.pixels() {
        println!("x: {x}, y: {y}");

        let color = color.to_rgb();
        match color.0 {
            [r, g, b] => {
                println!("r: {r}, g: {g}, b: {b}");
            }
        }

        if x > 2 {
            break;
        }
    }
}