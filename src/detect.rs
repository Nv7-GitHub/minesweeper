use std::{thread, time::Duration};
use std::io::ErrorKind::WouldBlock;
use opencv::prelude::*;

use super::*;
use image::DynamicImage::ImageRgba8;
use scrap::{Capturer, Display};

pub fn detect() -> [[u8; COLS]; ROWS] {
  // Init capture
  let display = Display::primary().unwrap();
  let mut cap = Capturer::new(display).unwrap();
  let (w, h) = (cap.width(), cap.height());

  // Take screenshot
  loop {
    // Get frame
    let buf = match cap.frame() {
      Ok(buffer) => buffer,
      Err(error) => {
        if error.kind() == WouldBlock {
          thread::sleep(Duration::new(1, 0)/60);
          continue;
        } else {
          panic!("{}", error);
        }
      }
    };

    // Flip the ARGB image into BGRA
    let mut bitflipped = Vec::with_capacity(w * h * 4);
    let stride = buf.len() / h;
    for y in 0..h {
      for x in 0..w {
          let i = stride * y + 4 * x;
          bitflipped.extend_from_slice(&[
              buf[i + 2],
              buf[i + 1],
              buf[i],
              255,
          ]);
      }
    }

    // Convert to image
    let img = image::ImageBuffer::from_raw(w as u32, h as u32, bitflipped).unwrap();
    let img = ImageRgba8(img);

    // Process
    img.save("image.png").unwrap();

    break;
  }


  [[0; COLS]; ROWS]
}