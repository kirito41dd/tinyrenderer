#![allow(unused_variables)]
#![allow(dead_code)]
use image::{imageops::flip_vertical_in_place, ImageBuffer, Rgba};

mod draw;

const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
const RED: Rgba<u8> = Rgba([255, 0, 0, 255]);
const GREEN: Rgba<u8> = Rgba([0, 255, 0, 255]);
const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);

fn main() {
    let (width, height) = (200, 200);
    let mut image = ImageBuffer::<Rgba<u8>, _>::from_pixel(width, height, BLACK);

    let t0 = glm::ivec2(10, 10);
    let t1 = glm::ivec2(100, 30);
    let t2 = glm::ivec2(190, 160);
    draw::triangle(t0, t1, t2, &mut image, RED);

    flip_vertical_in_place(&mut image);
    image.save("a.png").unwrap();
}
