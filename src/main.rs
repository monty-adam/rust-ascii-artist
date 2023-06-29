//! An example of opening an image.
extern crate image;
extern crate termion;

use image::{imageops::FilterType, GenericImageView, Pixel};
use std::{
    env,
    io::{stdout, Write},
    path::Path,
};
use termion::{clear, color as tty_color, raw::IntoRawMode};

const NUM_OF_FRAMES: u32 = 4;

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let im = match image::open(&Path::new(&file)) {
        Ok(buff) => buff.into_rgb8(),
        Err(err) => panic!("Failed to read image file: {:?}", err),
    };

    // We go to raw mode to make the control over the terminal more fine-grained.
    let mut stdout = stdout().into_raw_mode().unwrap();

    let termsize = termion::terminal_size().ok();
    let (termwidth, _termheight) = termsize.unwrap();

    // let im = if im.width() > im.height() {
    //     im.rotate90()
    // } else {
    //     im
    // };
    // let im = im.resize_exact(
    //     u32::try_from(termwidth)
    //         .map(|w| ((im.height() / im.width()) * w))
    //         .unwrap(),
    //     u32::try_from(termwidth / 2).unwrap(),
    //     FilterType::Triangle,
    // );

    let mut scenes: Vec<String> = Vec::with_capacity(usize::try_from(NUM_OF_FRAMES).unwrap());
    let total_pixels = im.height() * im.width();
    for (x, y, color) in im.pixels() {
        let x = x + 1
        let y = y + 1
        let mut ascii_art = if total_pixels % x * y == 0 {
            return String::new()
        }
        let ascii = match color.0 {
            [red, green, blue] => {
                format!(
                    "{}{}\u{2588} ",
                    termion::cursor::Goto(
                        u16::try_from(x).unwrap(),
                        u16::try_from(y).unwrap()
                    ),
                    tty_color::Fg(tty_color::Rgb(red, green, blue))
                )
            }
        };
        ascii_art.push_str(&ascii);
    }

    for scene in & scenes {
        write!(&mut stdout, "{}", clear::All).unwrap();
        stdout.write(scene.as_bytes()).unwrap();
    }
}
