//! An example of opening an image.
extern crate image;
extern crate termion;

use std::{
    env,
    io::{stdout, Write},
    path::Path,
    {thread, time},
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
        Ok(buff) => buff,
        Err(err) => panic!("Failed to read image file: {:?}", err),
    };
    let im = im.into_rgb8();

    let termsize = termion::terminal_size().ok();
    let (_termwidth, _termheight) = termsize.unwrap();

    let total_pixels = im.width() * im.height();
    println!("Total Pixels: {total_pixels}");

    let mut frames: Vec<String> = Vec::with_capacity(NUM_OF_FRAMES as usize);
    let mut pixels = im.enumerate_pixels();
    for _frame in 0..NUM_OF_FRAMES {
        let mut ascii_art = String::new();

        for y in 0..(im.height() / NUM_OF_FRAMES) {
            for x in 0..(im.width()) {
                let (_x, _y, color) = pixels.next().unwrap();
                let x = 1 + x;
                let y = 1 + y;
                let ascii = match color.0 {
                    [red, green, blue] => {
                        format!(
                            "{}{}\u{2588} ",
                            termion::cursor::Goto(x as u16, y as u16),
                            tty_color::Fg(tty_color::Rgb(red, green, blue))
                        )
                    }
                };

                ascii_art.push_str(&ascii);
            }
        }

        frames.push(ascii_art);
    }

    // We go to raw mode to make the control over the terminal more fine-grained.
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(&mut stdout, "{}", clear::All).unwrap();
    let ten_millis = time::Duration::from_millis(250);
    while let Some(frame) = frames.pop() {
        stdout.write(frame.as_bytes()).unwrap();
        thread::sleep(ten_millis);
    }
}
