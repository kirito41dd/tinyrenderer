use std::mem::swap;

use image::GenericImage;

// 画一个三角形
pub fn triangle<I: GenericImage>(
    mut t0: glm::Vec2,
    mut t1: glm::Vec2,
    mut t2: glm::Vec2,
    image: &mut I,
    color: I::Pixel,
) {
    if t0.y == t1.y && t0.y == t2.y {
        return;
    }

    // 按y坐标排序
    if t0.y > t1.y {
        swap(&mut t0, &mut t1);
    }
    if t0.y > t2.y {
        swap(&mut t0, &mut t2);
    }
    if t1.y > t2.y {
        swap(&mut t1, &mut t2);
    }

    let total_height = t2.y - t0.y;
    // 同时画两部分
    for i in 0..=total_height as i32 {
        let second_half = if i > (t1.y - t0.y) as i32 || t1.y == t0.y {
            true
        } else {
            false
        };
        let segment_height = if second_half {
            t2.y - t1.y
        } else {
            t1.y - t0.y
        };
        let alpha = i as f32 / total_height as f32;
        let beta =
            (i as f32 - if second_half { t1.y - t0.y } else { 0. }) as f32 / segment_height as f32; // be careful with divisions by zero

        let mut a = t0 + (t2 - t0) * alpha;
        let mut b = if second_half {
            t1 + (t2 - t1) * beta
        } else {
            t0 + (t1 - t0) * beta
        };
        if a.x > b.x {
            swap(&mut a, &mut b);
        }
        for j in a.x as i32..=b.x as i32 {
            image.put_pixel(j as u32, (t0.y + i as f32) as u32, color);
        }
    }
}

// 扫描线画一个三角形，分上下两部分
pub fn triangle_with_2_step<I: GenericImage>(
    mut t0: glm::Vec2,
    mut t1: glm::Vec2,
    mut t2: glm::Vec2,
    image: &mut I,
    color: I::Pixel,
) {
    if t0.y == t1.y && t0.y == t2.y {
        return;
    }

    // 按y坐标排序
    if t0.y > t1.y {
        swap(&mut t0, &mut t1);
    }
    if t0.y > t2.y {
        swap(&mut t0, &mut t2);
    }
    if t1.y > t2.y {
        swap(&mut t1, &mut t2);
    }

    let total_height = t2.y - t0.y;

    // 下半部分
    // y in t0.y -> t1.y
    for y in t0.y as i32..=t1.y as i32 {
        let segment_height = t1.y - t0.y + 1.;
        let alpha = (y as f32 - t0.y) as f32 / total_height as f32;
        let beta = (y as f32 - t0.y) as f32 / segment_height as f32; // be careful with divisions by zero

        let mut a = t0 + (t2 - t0) * alpha;
        let mut b = t0 + (t1 - t0) * beta;
        if a.x > b.x {
            swap(&mut a, &mut b);
        }
        for j in a.x as i32..=b.x as i32 {
            image.put_pixel(j as u32, y as u32, color);
        }
    }

    // 上半部分
    for y in t1.y as i32..=t2.y as i32 {
        let segment_height = t2.y - t1.y + 1.;
        let alpha = (y as f32 - t0.y) / total_height;
        let beta = (y as f32 - t1.y) / segment_height;
        let mut a = t0 + (t2 - t0) * alpha;
        let mut b = t1 + (t2 - t1) * beta;
        if a.x > b.x {
            swap(&mut a, &mut b);
        }
        for j in a.x as i32..=b.x as i32 {
            image.put_pixel(j as u32, y as u32, color);
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
