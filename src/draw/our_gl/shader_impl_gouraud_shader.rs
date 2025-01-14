use glm::{Mat4, Vec3};
use image::{ImageBuffer, Rgba};
use num::One;
use obj::TexturedVertex;

use super::IShader;

pub struct GouraudShader<'a> {
    model: &'a obj::Obj<TexturedVertex, u32>,
    diffuse: &'a ImageBuffer<Rgba<u8>, Vec<u8>>,
    varying_intensity: glm::Vec3, // 三个顶点的光照强度，由顶点着色器写入，由片段着色器读取
    varying_uv: glm::Mat3,        // 三个顶点的纹理坐标
    view_port: Mat4,
    projection: Mat4,
    model_view: Mat4,
    light_dir: Vec3,
}

impl<'a> GouraudShader<'a> {
    pub fn new(
        model: &'a obj::Obj<TexturedVertex, u32>,
        diffuse: &'a ImageBuffer<Rgba<u8>, Vec<u8>>,
        model_view: Mat4,
        projection: Mat4,
        view_port: Mat4,
        light_dir: Vec3,
    ) -> Self {
        Self {
            model,
            varying_intensity: glm::Vec3::one(),
            view_port,
            projection,
            model_view,
            light_dir,
            diffuse,
            varying_uv: glm::Mat3::one(),
        }
    }
}

impl<'a> IShader for GouraudShader<'a> {
    fn vertex(&mut self, i_face: usize, nth_vert: usize) -> glm::Vec4 {
        let i_vert = self.model.indices[i_face * 3 + nth_vert];
        let vert = self.model.vertices[i_vert as usize];
        let normal = Vec3::from_array(&vert.normal); // 顶点法向量
        let v = Vec3::from_array(&vert.position); // 顶点位置
        let uv = Vec3::from_array(&vert.texture); // 纹理坐标
        let gl_v = self.view_port * self.projection * self.model_view * v.extend(1.);
        self.varying_intensity[nth_vert] = glm::dot(*normal, self.light_dir).max(0.); // 计算每个顶点的光照强度
        self.varying_uv.as_array_mut()[nth_vert] = uv.clone(); // 每一列是一个顶点出的纹理坐标
        gl_v
    }

    fn fragment(&mut self, bar: glm::Vec3, color: &mut image::Rgba<u8>) -> bool {
        let intensity = glm::dot(self.varying_intensity, bar); // 当前像素的插值强度，重心坐标计算相对三个顶点的强度
        let uv = self.varying_uv * bar; // 用重心坐标插值当前点的纹理坐标
        let px = self.diffuse.get_pixel(
            (uv.x * self.diffuse.width() as f32) as _,
            (uv.y * self.diffuse.height() as f32) as _,
        );
        let r = (px[0] as f32 * intensity) as u8;
        let g = (px[1] as f32 * intensity) as u8;
        let b = (px[2] as f32 * intensity) as u8;
        *color = image::Rgba([r, g, b, 255]);
        return false; // 不丢弃任何像素
    }
}
