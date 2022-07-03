#![allow(unused_variables)]
#![allow(dead_code)]
use image::{imageops::flip_vertical_in_place, ImageBuffer, Rgba};

use std::{fs::File, io::BufReader};

mod draw;

const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
const RED: Rgba<u8> = Rgba([255, 0, 0, 255]);
const GREEN: Rgba<u8> = Rgba([0, 255, 0, 255]);
const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);
fn main() {
    let (width, height) = (800, 800);
    let mut image = ImageBuffer::<Rgba<u8>, _>::from_pixel(width, height, BLACK);

    let input = BufReader::new(File::open("a.obj").unwrap());
    let model: obj::Obj = obj::load_obj(input).unwrap();
    for arr in model.indices.chunks(3) {
        for i in 0..3 {
            let v0 = model.vertices.get(arr[i] as usize).unwrap().position;
            let v1 = model
                .vertices
                .get(arr[(i + 1) % 3] as usize)
                .unwrap()
                .position;
            let x0 = ((v0[0] + 1.0) * (width - 1) as f32 / 2.0) as i32;
            let y0 = ((v0[1] + 1.0) * (height - 1) as f32 / 2.0) as i32;
            let x1 = ((v1[0] + 1.0) * (width - 1) as f32 / 2.0) as i32;
            let y1 = ((v1[1] + 1.0) * (height - 1) as f32 / 2.0) as i32;
            draw::line(glm::ivec2(x0, y0), glm::ivec2(x1, y1), &mut image, WHITE);
        }
    }

    flip_vertical_in_place(&mut image);
    image.save("a.png").unwrap();
}
