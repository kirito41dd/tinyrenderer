use std::mem::swap;

use image::GenericImage;

pub fn line<I: GenericImage>(t0: glm::IVec2, t1: glm::IVec2, image: &mut I, color: I::Pixel) {
    line_v5(t0, t1, image, color);
}

//  画一条线，继续优化cpu，不用浮点数
fn line_v5<I: GenericImage>(mut a: glm::IVec2, mut b: glm::IVec2, image: &mut I, color: I::Pixel) {
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

//  画一条线，优化cpu
fn line_v4<I: GenericImage>(mut a: glm::IVec2, mut b: glm::IVec2, image: &mut I, color: I::Pixel) {
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
    let derror = (dy as f64 / dx as f64).abs();
    let mut error = 0.0;
    let mut y = a.y;
    for x in a.x..=b.x {
        if steep {
            image.put_pixel(y as u32, x as u32, color);
        } else {
            image.put_pixel(x as u32, y as u32, color);
        }
        error += derror;
        if error > 0.5 {
            y += if b.y > a.y { 1 } else { -1 };
            error -= 1.0;
        }
    }
}

//  画一条线，没问题就是费cpu（循环里的除法）
fn line_v3<I: GenericImage>(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    image: &mut I,
    color: I::Pixel,
) {
    let mut steep = false;
    if (x0 - x1).abs() < (y0 - y1).abs() {
        // if the line is steep, we transpose the image
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
        steep = true;
    }
    if x0 > x1 {
        // make it left−to−right
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }
    for x in x0..=x1 {
        let t = (x - x0) as f64 / (x1 - x0) as f64;
        let y = (y0 as f64 * (1.0 - t)) + (y1 as f64 * t);
        if steep {
            image.put_pixel(y as u32, x as u32, color);
        } else {
            image.put_pixel(x as u32, y as u32, color);
        }
    }
}

//  画一条线，有bug，x0如果大于x1什么都画不出来，太陡会有断点
fn line_v2<I: GenericImage>(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut I, color: I::Pixel) {
    for x in x0..=x1 {
        let t = (x - x0) as f64 / (x1 - x0) as f64;
        let y = (y0 as f64 * (1.0 - t)) + (y1 as f64 * t);
        println!("{} {} {}", x, y, t);
        image.put_pixel(x as u32, y as u32, color)
    }
}

// 画一条线，步长太大会导致线段不连续
fn line_v1<I: GenericImage>(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut I, color: I::Pixel) {
    let mut t = 0.0;
    while t < 1.0 {
        let x = (x0 as f64 + (x1 as f64 - x0 as f64) * t) as u32;
        let y = (y0 as f64 + (y1 as f64 - y0 as f64) * t) as u32;
        image.put_pixel(x, y, color);
        t += 0.01;
    }
}
