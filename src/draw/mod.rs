use std::mem::swap;

use image::GenericImage;

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
