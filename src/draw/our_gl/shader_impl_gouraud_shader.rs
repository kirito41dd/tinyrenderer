use glm::{Mat4, Vec3};
use num::One;
use obj::TexturedVertex;

use super::IShader;

pub struct GouraudShader<'a> {
    model: &'a obj::Obj<TexturedVertex, u32>,
    varying_intensity: glm::Vec3, // 强度变化，由顶点着色器写入，由片段着色器读取
    view_port: Mat4,
    projection: Mat4,
    model_view: Mat4,
    light_dir: Vec3,
}

impl<'a> GouraudShader<'a> {
    pub fn new(
        model: &'a obj::Obj<TexturedVertex, u32>,
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
        }
    }
}

impl<'a> IShader for GouraudShader<'a> {
    fn vertex(&mut self, i_face: usize, nth_vert: usize) -> glm::Vec4 {
        let i_vert = self.model.indices[i_face * 3 + nth_vert];
        let vert = self.model.vertices[i_vert as usize];
        let normal = Vec3::from_array(&vert.normal);
        let v = Vec3::from_array(&vert.position);
        let gl_v = self.view_port * self.projection * self.model_view * v.extend(1.);
        self.varying_intensity[nth_vert] = glm::dot(*normal, self.light_dir).max(0.); // 计算每个顶点的光照强度
        gl_v
    }

    fn fragment(&mut self, bar: glm::Vec3, color: &mut image::Rgba<u8>) -> bool {
        let intensity = glm::dot(self.varying_intensity, bar); // 当前像素的插值强度，重心坐标计算相对三个顶点的强度
        let x = (255. * intensity) as u8;
        *color = image::Rgba([x, x, x, 255]);
        return false; // 不丢弃任何像素
    }
}
