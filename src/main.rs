#![allow(unused_variables)]
#![allow(dead_code)]
use std::{fs::File, io::BufReader};

use draw::{
    lookat,
    our_gl::{
        shader_impl_gouraud_shader::GouraudShader, shader_impl_phong_shader::PhongShader, IShader,
    },
    triangle_with_shader, viewport,
};
use image::{imageops::flip_vertical_in_place, ImageBuffer, Luma, Rgba};
use num::Zero;

mod draw;

const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
const RED: Rgba<u8> = Rgba([255, 0, 0, 255]);
const GREEN: Rgba<u8> = Rgba([0, 255, 0, 255]);
const BLUE: Rgba<u8> = Rgba([0, 0, 255, 255]);
const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);

/// 齐次坐标系中的点投影到3d
/// 点坐标需要除以w
pub fn v4p2v3(v: glm::Vec4) -> glm::Vec3 {
    glm::vec3(v.x / v.w, v.y / v.w, v.z / v.w)
}

/// 齐次坐标系中的向量投影到3d
/// 向量坐标不需要除以w
pub fn vec4_to_3(v: glm::Vec4) -> glm::Vec3 {
    glm::vec3(v.x, v.y, v.z)
}

fn main() {
    let eye = glm::vec3(1., 1., 3.); // camera
    let center = glm::vec3(0., 0., 0.);
    let up = glm::vec3(0., 1., 0.);
    let light_dir = glm::normalize(glm::vec3(1., 1., 0.9));
    let (width, height) = (800, 800);
    let mut diffus = image::open("obj/african_head/african_head_diffuse.tga")
        .unwrap()
        .to_rgba8();
    let mut diffus_nm = image::open("obj/african_head/african_head_nm.tga")
        .unwrap()
        .to_rgba8();
    let _ = flip_vertical_in_place(&mut diffus);
    let _ = flip_vertical_in_place(&mut diffus_nm);
    let mut image = ImageBuffer::<Rgba<u8>, _>::from_pixel(width, height, BLACK);
    let mut zbuffer = ImageBuffer::<Luma<u8>, _>::from_pixel(width, height, Luma([0]));
    //let mut zbuffer = vec![f32::MIN; (image.width() * image.height()) as usize]; // 注意一定初始化为最小值

    let input = BufReader::new(File::open("obj/african_head/african_head.obj").unwrap());
    let model = obj::load_obj::<obj::TexturedVertex, _, u32>(input).unwrap();

    let model_view = lookat(eye, center, up);

    #[rustfmt::skip]
    let projection = glm::mat4(
        1., 0., 0., 0.,
        0., 1., 0., 0.,
        0., 0., 1., -1./ glm::distance(eye, center),
        0., 0., 0., 1.);

    let view_port = viewport(
        width as i32 / 8,
        height as i32 / 8,
        width as i32 * 3 / 4,
        height as i32 * 3 / 4,
    );

    let m = view_port * projection * model_view;

    let mut _shader = GouraudShader::new(
        &model, &diffus, model_view, projection, view_port, light_dir,
    );
    let mut shader = PhongShader::new(&model, &diffus, &diffus_nm, m, light_dir);
    for i in 0..model.indices.len() / 3 {
        let mut screen_coords: [glm::Vec4; 3] = [glm::Vec4::zero(); 3];
        for j in 0..3 {
            screen_coords[j] = shader.vertex(i, j);
        }
        triangle_with_shader(
            screen_coords[0],
            screen_coords[1],
            screen_coords[2],
            &mut shader,
            &mut image,
            &mut zbuffer,
        );
    }

    flip_vertical_in_place(&mut image);
    image.save("a.png").unwrap();
    flip_vertical_in_place(&mut zbuffer);
    zbuffer.save("b.png").unwrap();
}
