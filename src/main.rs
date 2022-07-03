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

    let t0 = [
        glm::vec2(10.0, 70.0),
        glm::vec2(50.0, 160.0),
        glm::vec2(70.0, 80.0),
    ];
    let t1 = [
        glm::vec2(180.0, 50.0),
        glm::vec2(150.0, 1.0),
        glm::vec2(70.0, 180.0),
    ];
    let t2 = [
        glm::vec2(180.0, 150.0),
        glm::vec2(120.0, 160.0),
        glm::vec2(130.0, 180.0),
    ];
    draw::triangle(t0[0], t0[1], t0[2], &mut image, RED);
    draw::triangle(t1[0], t1[1], t1[2], &mut image, WHITE);
    draw::triangle(t2[0], t2[1], t2[2], &mut image, GREEN);

    flip_vertical_in_place(&mut image);
    image.save("a.png").unwrap();
}
