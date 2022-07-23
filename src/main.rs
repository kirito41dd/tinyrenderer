#![allow(unused_variables)]
#![allow(dead_code)]

use image::{imageops::flip_vertical_in_place, ImageBuffer, Rgba};

mod draw;

const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
const RED: Rgba<u8> = Rgba([255, 0, 0, 255]);
const GREEN: Rgba<u8> = Rgba([0, 255, 0, 255]);
const BLUE: Rgba<u8> = Rgba([0, 0, 255, 255]);
const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);

fn main() {
    let (width, height) = (800, 20);
    let mut image = ImageBuffer::<Rgba<u8>, _>::from_pixel(width, height, BLACK);

    let mut ybuffer = vec![0; 800];
    draw::resterize(
        glm::ivec2(330, 463),
        glm::ivec2(594, 200),
        &mut image,
        &mut ybuffer,
        BLUE,
    );
    draw::resterize(
        glm::ivec2(120, 434),
        glm::ivec2(444, 400),
        &mut image,
        &mut ybuffer,
        GREEN,
    );
    draw::resterize(
        glm::ivec2(20, 34),
        glm::ivec2(744, 400),
        &mut image,
        &mut ybuffer,
        RED,
    );

    flip_vertical_in_place(&mut image);
    image.save("a.png").unwrap();
}
