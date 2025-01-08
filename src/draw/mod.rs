use std::mem::swap;

use glm::Vec3;
use image::{GenericImage, Luma, Rgba};
use our_gl::IShader;

use crate::v4p2v3;

pub mod our_gl;

// 求重心坐标
fn barycentric(a: glm::Vec3, b: glm::Vec3, c: glm::Vec3, p: glm::Vec3) -> glm::Vec3 {
    let ab = b - a;
    let ac = c - a;
    let pa = a - p;

    let u = glm::cross(
        glm::vec3(ab.x as f32, ac.x as f32, pa.x as f32),
        glm::vec3(ab.y as f32, ac.y as f32, pa.y as f32),
    );

    // z是0，这种情况是因为三角形三个顶点在一条直线上，不是合法三角形
    // 这种情况返回一个负值
    if u.z.abs() <= f32::EPSILON {
        return glm::vec3(-1., 1., 1.);
    }

    // vec(x,y,z)/z -> (u,v,1) -> (1-u-v, u, v)
    return glm::vec3(1. - ((u.x + u.y) / u.z) as f32, u.x / u.z, u.y / u.z);
}

pub fn triangle<I: GenericImage>(
    t0: glm::Vec3,
    t1: glm::Vec3,
    t2: glm::Vec3,
    image: &mut I,
    color: I::Pixel,
    zbuffer: &mut [f32],
) {
    let bboxmin = glm::vec2(
        t0.x.min(t1.x).min(t2.x).max(0.),
        t0.y.min(t1.y).min(t2.y).max(0.),
    );
    let bboxmax = glm::vec2(
        t0.x.max(t1.x).max(t2.x).min(image.width() as f32 - 1.),
        t0.y.max(t1.y).max(t2.y).min(image.height() as f32 - 1.),
    );

    for px in bboxmin.x as i32..=bboxmax.x as i32 {
        for py in bboxmin.y as i32..=bboxmax.y as i32 {
            let bc_screen = barycentric(t0, t1, t2, glm::vec3(px as f32, py as f32, 0.));

            if bc_screen.x < 0. || bc_screen.y < 0. || bc_screen.z < 0. {
                continue;
            }
            // 计算z值
            let pz = glm::dot(glm::vec3(t0.z, t1.z, t2.z), bc_screen);
            let idx = px + py * image.width() as i32;
            if zbuffer[idx as usize] <= pz {
                zbuffer[idx as usize] = pz;
                image.put_pixel(px as u32, py as u32, color);
            }
        }
    }
}

pub fn triangle_with_texture<I: GenericImage<Pixel = Rgba<u8>>>(
    a: glm::Vec3,
    b: glm::Vec3,
    c: glm::Vec3,
    ta: glm::Vec3,
    tb: glm::Vec3,
    tc: glm::Vec3,
    image: &mut I,
    intensity: f32,
    zbuffer: &mut [f32],
    diffuse: &I,
) {
    let bboxmin = glm::vec2(a.x.min(b.x).min(c.x).max(0.), a.y.min(b.y).min(c.y).max(0.));
    let bboxmax = glm::vec2(
        a.x.max(b.x).max(c.x).min(image.width() as f32 - 1.),
        a.y.max(b.y).max(c.y).min(image.height() as f32 - 1.),
    );

    for px in bboxmin.x as i32..=bboxmax.x as i32 {
        for py in bboxmin.y as i32..=bboxmax.y as i32 {
            let bc_screen = barycentric(a, b, c, glm::vec3(px as f32, py as f32, 0.));

            if bc_screen.x < 0. || bc_screen.y < 0. || bc_screen.z < 0. {
                continue;
            }
            // 计算z值
            let pz = glm::dot(glm::vec3(a.z, b.z, c.z), bc_screen);
            // 计算纹理插值
            let tx = glm::dot(glm::vec3(ta.x, tb.x, tc.x), bc_screen) * diffuse.width() as f32;
            let ty = glm::dot(glm::vec3(ta.y, tb.y, tc.y), bc_screen) * diffuse.height() as f32;
            let idx = px + py * image.width() as i32;
            let pi: Rgba<u8> = diffuse.get_pixel(tx as u32, ty as u32);
            if zbuffer[idx as usize] <= pz {
                zbuffer[idx as usize] = pz;
                image.put_pixel(
                    px as u32,
                    py as u32,
                    Rgba([
                        (pi.0[0] as f32 * intensity) as u8,
                        (pi.0[1] as f32 * intensity) as u8,
                        (pi.0[2] as f32 * intensity) as u8,
                        255,
                    ]),
                );
            }
        }
    }
}

/// 注意现在输入的顶点坐标是齐次坐标
pub fn triangle_with_shader<
    I: GenericImage<Pixel = Rgba<u8>>,
    I2: GenericImage<Pixel = Luma<u8>>,
    S: IShader,
>(
    a_4d: glm::Vec4,
    b_4d: glm::Vec4,
    c_4d: glm::Vec4,
    shader: &mut S,
    image: &mut I,
    zbuffer: &mut I2,
) {
    let a = v4p2v3(a_4d);
    let b = v4p2v3(b_4d);
    let c = v4p2v3(c_4d);
    let bboxmin = glm::vec2(a.x.min(b.x).min(c.x).max(0.), a.y.min(b.y).min(c.y).max(0.));
    let bboxmax = glm::vec2(
        a.x.max(b.x).max(c.x).min(image.width() as f32 - 1.),
        a.y.max(b.y).max(c.y).min(image.height() as f32 - 1.),
    );

    for px in bboxmin.x as i32..=bboxmax.x as i32 {
        for py in bboxmin.y as i32..=bboxmax.y as i32 {
            let bc_screen = barycentric(a, b, c, glm::vec3(px as f32, py as f32, 0.));
            // 留意下这里，z和w都使用齐次坐标算的
            let z = glm::dot(glm::vec3(a_4d.z, b_4d.z, c_4d.z), bc_screen);
            let w = glm::dot(glm::vec3(a_4d.w, b_4d.w, c_4d.w), bc_screen);

            let frag_depth = (z / w + 0.5) as u8;
            let frag_depth = frag_depth.min(255).max(0);

            if bc_screen.x < 0. || bc_screen.y < 0. || bc_screen.z < 0. {
                continue;
            }
            let mut color = image::Rgba([0; 4]);
            let discard = shader.fragment(bc_screen, &mut color);
            let idx = px + py * image.width() as i32;
            let zb: &mut Luma<u8> = zbuffer.get_pixel_mut(px as _, py as _);
            if zb.0[0] <= frag_depth {
                zb.0[0] = frag_depth;
                if !discard {
                    image.put_pixel(px as u32, py as u32, color);
                }
            }
        }
    }
}

pub fn line<I: GenericImage>(mut a: glm::IVec2, mut b: glm::IVec2, image: &mut I, color: I::Pixel) {
    let mut steep = false;
    if (a.x - b.x).abs() < (a.y - b.y).abs() {
        // if the line is steep, we transpose the image
        swap(&mut a.x, &mut a.y);
        swap(&mut b.x, &mut b.y);
        steep = true;
    }
    if a.x > b.x {
        // make it left−to−right
        swap(&mut a, &mut b);
    }
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let derror = dy.abs() * 2;
    let mut error = 0;
    let mut y = a.y;
    for x in a.x..=b.x {
        if steep {
            image.put_pixel(y as u32, x as u32, color);
        } else {
            image.put_pixel(x as u32, y as u32, color);
        }
        error += derror;
        if error > dx {
            y += if b.y > a.y { 1 } else { -1 };
            error -= dx * 2;
        }
    }
}

pub fn resterize<I: GenericImage>(
    mut a: glm::IVec2,
    mut b: glm::IVec2,
    image: &mut I,
    ybuffer: &mut [i32],
    color: I::Pixel,
) {
    if (a.x - b.x).abs() < (a.y - b.y).abs() {
        swap(&mut a.x, &mut a.y);
        swap(&mut b.x, &mut b.y);
    }
    for x in a.x..=b.x {
        let t = (x - a.x) as f32 / (b.x - a.x) as f32;
        let y = a.y as f32 * (1. - t) + b.y as f32 * t;

        if ybuffer[x as usize] < y as i32 {
            ybuffer[x as usize] = y as i32;
            for i in 0..image.height() {
                image.put_pixel(x as u32, i, color);
            }
        }
    }
}

// z -> (0,0,-1)
// eye 摄像机位置 center 焦点 up视角上方
pub fn lookat(eye: glm::Vec3, center: glm::Vec3, up: Vec3) -> glm::Matrix4<f32> {
    let z = glm::normalize(eye - center); // 向量ce
    let x = glm::normalize(glm::cross(up, z)); // 同时垂直于 up和z的向量
    let y = glm::normalize(glm::cross(z, x));
    let minv = glm::mat4(
        x.x, y.x, z.x, 0., x.y, y.y, z.y, 0., x.z, y.z, z.z, 0., 0., 0., 0., 1.,
    );
    #[rustfmt::skip]
    // 这里平移为什么是用的center? 因为把摄像机移动回去这个动作，我们并没有定义原来的摄像机位置，所以不知道位移的向量
    // 但是原来的焦点可以认为是原点(0,0,0)，摄像机的位移和焦点位移是一样的，所以用center的坐标来计算
    // 这里如果用eye，就相当于假设原来摄像机在原点，结果也对就是看着比预想中远
    let tr = glm::mat4(
        1., 0., 0., 0.,
        0., 1., 0., 0.,
        0., 0., 1., 0.,
        -center.x, -center.y, -center.z, 1.,
    );
    minv * tr
}

pub fn viewport(x: i32, y: i32, w: i32, h: i32) -> glm::Matrix4<f32> {
    let (x, y, w, h) = (x as f32, y as f32, w as f32, h as f32);
    let d = 255.;
    #[rustfmt::skip]
    let m = glm::mat4(
        w/2., 0., 0., 0., 
        0., h/2., 0., 0., 
        0., 0., d/2., 0., 
        x+w/2., y+h/2., d/2., 1.,
    );
    m
}
