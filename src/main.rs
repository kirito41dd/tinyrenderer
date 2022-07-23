#![allow(unused_variables)]
#![allow(dead_code)]
use std::{fs::File, io::BufReader};

use image::{imageops::flip_vertical_in_place, ImageBuffer, Rgba};

mod draw;

const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
const RED: Rgba<u8> = Rgba([255, 0, 0, 255]);
const GREEN: Rgba<u8> = Rgba([0, 255, 0, 255]);
const BLUE: Rgba<u8> = Rgba([0, 0, 255, 255]);
const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);

fn main() {
    let (width, height) = (800, 800);
    let mut diffus = image::open("obj/african_head/african_head_diffuse.tga")
        .unwrap()
        .to_rgba8();
    let diffuse = flip_vertical_in_place(&mut diffus);
    let mut image = ImageBuffer::<Rgba<u8>, _>::from_pixel(width, height, BLACK);
    let mut zbuffer = vec![f32::MIN; (image.width() * image.height()) as usize]; // 注意一定初始化为最小值

    let input = BufReader::new(File::open("obj/african_head/african_head.obj").unwrap());
    let model = obj::load_obj::<obj::TexturedVertex, _, u32>(input).unwrap();
    let light_dir = glm::vec3(0., 0., -0.9);

    for arr in model.indices.chunks(3) {
        let (a, b, c, ta, tb, tc) = (
            model.vertices.get(arr[0] as usize).unwrap().position,
            model.vertices.get(arr[1] as usize).unwrap().position,
            model.vertices.get(arr[2] as usize).unwrap().position,
            model.vertices.get(arr[0] as usize).unwrap().texture,
            model.vertices.get(arr[1] as usize).unwrap().texture,
            model.vertices.get(arr[2] as usize).unwrap().texture,
        );
        let (a, b, c, ta, tb, tc) = (
            glm::vec3(a[0], a[1], a[2]),
            glm::vec3(b[0], b[1], b[2]),
            glm::vec3(c[0], c[1], c[2]),
            glm::vec3(ta[0], ta[1], ta[2]),
            glm::vec3(tb[0], tb[1], tb[2]),
            glm::vec3(tc[0], tc[1], tc[2]),
        );
        let (sa, sb, sc) = (
            glm::vec3(
                ((a.x + 1.) * (width) as f32) / 2. + 0.5,
                ((a.y + 1.) * (height) as f32) / 2. + 0.5,
                a.z,
            ),
            glm::vec3(
                ((b.x + 1.) * (width) as f32) / 2. + 0.5,
                ((b.y + 1.) * (height) as f32) / 2. + 0.5,
                b.z,
            ),
            glm::vec3(
                ((c.x + 1.) * (width) as f32) / 2. + 0.5,
                ((c.y + 1.) * (height) as f32) / 2. + 0.5,
                c.z,
            ),
        );

        let n = glm::cross(c - a, b - a);
        let n = glm::normalize(n);

        let intensity = glm::dot(light_dir, n);

        if intensity > 0. {
            // 既是光照强度，也能当面剔除用
            draw::triangle_with_texture(
                sa,
                sb,
                sc,
                ta,
                tb,
                tc,
                &mut image,
                intensity,
                &mut zbuffer,
                &diffus,
            );
        }
    }

    flip_vertical_in_place(&mut image);
    image.save("a.png").unwrap();
}
