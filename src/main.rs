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

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let im = image::open(&Path::new(&file)).unwrap();

    // We go to raw mode to make the control over the terminal more fine-grained.
    let mut stdout = stdout().into_raw_mode().unwrap();

    let termsize = termion::terminal_size().ok();
    let (termwidth, _termheight) = termsize.unwrap();

    let im = im.resize_exact(
        u32::try_from(termwidth)
            .map(|w| ((im.height() / im.width()) * w))
            .unwrap(),
        u32::try_from(termwidth / 2).unwrap(),
        FilterType::Triangle,
    );

    let mut ascii_art = String::new();
    for (x, y, color) in im.pixels() {
        let color = color.to_rgb();
        let ascii = match color.0 {
            [red, green, blue] => {
                format!(
                    "{}{}\u{2588} ",
                    termion::cursor::Goto(
                        u16::try_from(x + 1).unwrap(),
                        u16::try_from(y + 1).unwrap()
                    ),
                    tty_color::Fg(tty_color::Rgb(red, green, blue))
                )
            }
        };
        ascii_art.push_str(&ascii);
    }

    write!(&mut stdout, "{}", clear::All).unwrap();
    stdout.write(ascii_art.as_bytes()).unwrap();
}
