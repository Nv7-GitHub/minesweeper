use std::{thread, time::Duration};
use std::io::ErrorKind::WouldBlock;
use opencv::core;
use opencv::prelude::*;
use opencv::imgproc;
use opencv::imgcodecs;
use opencv::types;
use ordered_float::OrderedFloat;

use super::*;
use image::DynamicImage::ImageRgba8;
use scrap::{Capturer, Display};

const BUF: i32 = 5; // Buffer fromm edges

pub fn detect() -> (Vec<Vec<u8>>, (i32, i32), i32) { // (board, start, sqsize)
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

    // Convert to OpenCV
    let img = image::ImageBuffer::from_raw(w as u32, h as u32, bitflipped).unwrap();
    let img = ImageRgba8(img);
    let mut img_mat = Mat::new_rows_cols_with_default(img.height() as i32, img.width() as i32, core::Vec4b::typ(), core::Scalar::all(0.)).unwrap();
    img_mat.data_bytes_mut().unwrap().copy_from_slice(img.as_bytes());
    let mut img = Mat::default();
    imgproc::cvt_color(&img_mat, &mut img, imgproc::COLOR_RGBA2BGR, 0).unwrap();

    imgcodecs::imwrite("image.png", &img, &core::Vector::new()).unwrap();

    // Get bounding box of top
    let mut imghsv = Mat::default();
    imgproc::cvt_color(&img, &mut imghsv, imgproc::COLOR_BGR2HSV, 0).unwrap();
    let mut top = Mat::default();
    core::in_range(&imghsv, &core::Vec3b::from([48, 0, 0]), &core::Vec3b::from([52, 200, 255]), &mut top).unwrap();
    let mut cont = types::VectorOfMat::new();
    imgproc::find_contours(&top, &mut cont, imgproc::RETR_TREE, imgproc::CHAIN_APPROX_SIMPLE, core::Point::new(0, 0)).unwrap();
    let max = cont.iter().max_by_key(|x| OrderedFloat(imgproc::contour_area(x, false).unwrap())).unwrap();
    let rect = imgproc::bounding_rect(&max).unwrap();

    imgcodecs::imwrite("top.png", &top, &core::Vector::new()).unwrap();

    // Consts
    let sqsize = rect.width/COLS as i32;
    let start = (rect.x, rect.y + rect.height); // (x, y) of top-left of grid
    let nums = vec![
      (1 as u8, core::Vec3b::from([100, 0, 0]), core::Vec3b::from([110, 255, 255])),
      (2, core::Vec3b::from([55, 0, 0]), core::Vec3b::from([65, 255, 255])),
      (3, core::Vec3b::from([0, 50, 0]), core::Vec3b::from([5, 255, 255])),
      (4, core::Vec3b::from([140, 0, 0]), core::Vec3b::from([150, 255, 255])),
      (5, core::Vec3b::from([15, 215, 0]), core::Vec3b::from([25, 255, 255])),
      (6, core::Vec3b::from([90, 0, 0]), core::Vec3b::from([45, 255, 255])),
      (7, core::Vec3b::from([0, 0, 50]), core::Vec3b::from([0, 0, 70])),
      // TODO: 8
      (9, core::Vec3b::from([40, 100, 0]), core::Vec3b::from([45, 255, 255])), // Green
    ];

    // Create cropped squares
    let mut board = Vec::with_capacity(ROWS);
    for r in 0..ROWS {
      let mut row = Vec::with_capacity(COLS);
      for c in 0..COLS {
        // Crop & calc num
        let crop = Mat::roi(&img, core::Rect::new(start.0 as i32 + c as i32*sqsize + BUF, start.1 + r as i32*sqsize + BUF, sqsize - BUF, sqsize - BUF)).unwrap();
        let mut crophsv = Mat::default();
        imgproc::cvt_color(&crop, &mut crophsv, imgproc::COLOR_BGR2HSV, 0).unwrap();
        let mut num = 0;
        let mut max = 100; // 100 is min for it to count as not blank
        for det in nums.iter() {
          let mut thresh = Mat::default();
          core::in_range(&crophsv, &det.1, &det.2, &mut thresh).unwrap();
          let cnt = core::count_non_zero(&thresh).unwrap();
          if cnt > max { // Greater than max, update
            num = det.0;
            max = cnt;
          }
        }
        row.push(num);
      }
      board.push(row);
    }

    return (board, start, sqsize);
  }
}