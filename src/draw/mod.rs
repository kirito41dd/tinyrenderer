use std::mem::swap;

use image::GenericImage;

// 求重心坐标
fn barycentric(a: glm::IVec2, b: glm::IVec2, c: glm::IVec2, p: glm::IVec2) -> glm::Vec3 {
    let ab = b - a;
    let ac = c - a;
    let pa = a - p;

    let u = glm::cross(
        glm::vec3(ab.x as f32, ac.x as f32, pa.x as f32),
        glm::vec3(ab.y as f32, ac.y as f32, pa.y as f32),
    );

    // 因为传入坐标都是整数，所以z小于1就意味着z是0，这种情况是因为三角形三个顶点在一条直线上，不是合法三角形
    // 这种情况返回一个负值
    if u.z.abs() < 1. {
        return glm::vec3(-1., 1., 1.);
    }

    // vec(x,y,z)/z -> (u,v,1) -> (1-u-v, u, v)
    return glm::vec3(1. - ((u.x + u.y) / u.z) as f32, u.x / u.z, u.y / u.z);
}

pub fn triangle<I: GenericImage>(
    t0: glm::IVec2,
    t1: glm::IVec2,
    t2: glm::IVec2,
    image: &mut I,
    color: I::Pixel,
) {
    let bboxmin = glm::ivec2(t0.x.min(t1.x).min(t2.x), t0.y.min(t1.y).min(t2.y));
    let bboxmax = glm::ivec2(t0.x.max(t1.x).max(t2.x), t0.y.max(t1.y).max(t2.y));
    let a = t0[1];

    for px in bboxmin.x..=bboxmax.x {
        for py in bboxmin.y..=bboxmax.y {
            let bc_screen = barycentric(t0, t1, t2, glm::ivec2(px, py));

            if bc_screen.x < 0. || bc_screen.y < 0. || bc_screen.z < 0. {
                continue;
            }
            image.put_pixel(px as u32, py as u32, color);
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
