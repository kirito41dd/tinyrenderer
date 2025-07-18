use glm::{Mat3, Mat4, Vec3};
use num::Zero;
use obj::TexturedVertex;

use crate::v4p2v3;

use super::IShader;

pub struct ShadowShader<'a> {
    model: &'a obj::Obj<TexturedVertex, u32>,
    varying_tri: Mat3, // 三个顶点的屏幕坐标
    view_port: Mat4,
    projection: Mat4,
    model_view: Mat4,
}

impl<'a> ShadowShader<'a> {
    pub fn new(
        model: &'a obj::Obj<TexturedVertex, u32>,
        model_view: Mat4,
        projection: Mat4,
        view_port: Mat4,
    ) -> Self {
        Self {
            model,
            varying_tri: Mat3::zero(),
            view_port,
            projection,
            model_view,
        }
    }
}

impl<'a> IShader for ShadowShader<'a> {
    fn vertex(&mut self, i_face: usize, nth_vert: usize) -> glm::Vec4 {
        let i_vert = self.model.indices[i_face * 3 + nth_vert];
        let vert = self.model.vertices[i_vert as usize];
        let v = Vec3::from_array(&vert.position); // 顶点位置
        let gl_v = self.view_port * self.projection * self.model_view * v.extend(1.);
        self.varying_tri.as_array_mut()[nth_vert] = v4p2v3(gl_v);
        gl_v
    }

    fn fragment(&mut self, bar: glm::Vec3, color: &mut image::Rgba<u8>) -> bool {
        let p = self.varying_tri * bar; // 当前像素的插值位置
        let depth = 2000.;
        let r = (255. * p.z / depth) as u8;
        let g = (255. * p.z / depth) as u8;
        let b = (255. * p.z / depth) as u8;
        *color = image::Rgba([r, g, b, 255]); // 设置当前像素颜色为阴影颜色,深度越小颜色越潜
        return false; // 不丢弃任何像素
    }
}
