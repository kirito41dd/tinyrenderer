#![allow(unused_variables)]
#![allow(dead_code)]
use std::{fs::File, io::BufReader};

use image::{imageops::flip_vertical_in_place, ImageBuffer, Rgba};

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
    let light_dir = glm::vec3(0., 0., -0.9);
    for arr in model.indices.chunks(3) {
        let (a, b, c) = (
            model.vertices.get(arr[0] as usize).unwrap().position,
            model.vertices.get(arr[1] as usize).unwrap().position,
            model.vertices.get(arr[2] as usize).unwrap().position,
        );
        let (a, b, c) = (
            glm::vec3(a[0], a[1], a[2]),
            glm::vec3(b[0], b[1], b[2]),
            glm::vec3(c[0], c[1], c[2]),
        );
        let (sa, sb, sc) = (
            glm::ivec2(
                (((a.x + 1.) * (width - 1) as f32) / 2.) as i32,
                (((a.y + 1.) * (height - 1) as f32) / 2.) as i32,
            ),
            glm::ivec2(
                (((b.x + 1.) * (width - 1) as f32) / 2.) as i32,
                (((b.y + 1.) * (height - 1) as f32) / 2.) as i32,
            ),
            glm::ivec2(
                (((c.x + 1.) * (width - 1) as f32) / 2.) as i32,
                (((c.y + 1.) * (height - 1) as f32) / 2.) as i32,
            ),
        );

        let n = glm::cross(c - a, b - a);
        let n = glm::normalize(n);

        let intensity = glm::dot(light_dir, n);

        if intensity > 0. {
            // 既是光照强度，也能当面剔除用
            draw::triangle(
                sa,
                sb,
                sc,
                &mut image,
                Rgba([
                    (255. * intensity) as u8,
                    (255. * intensity) as u8,
                    (255. * intensity) as u8,
                    255,
                ]),
                // Rgba([
                //     rand::random::<u8>() % 255,
                //     rand::random::<u8>() % 255,
                //     rand::random::<u8>() % 255,
                //     255,
                // ]),
            );
        }
    }

    flip_vertical_in_place(&mut image);
    image.save("a.png").unwrap();
}
