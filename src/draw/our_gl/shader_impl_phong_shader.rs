use glm::{GenMat, GenSquareMat, Mat4, Vec3};
use image::{ImageBuffer, Rgba};
use num::One;
use obj::TexturedVertex;

use crate::vec4_to_3;

use super::IShader;

pub struct PhongShader<'a> {
    model: &'a obj::Obj<TexturedVertex, u32>,
    diffuse: &'a ImageBuffer<Rgba<u8>, Vec<u8>>,
    diffuse_nm: &'a ImageBuffer<Rgba<u8>, Vec<u8>>, // 法线贴图
    varying_uv: glm::Mat3,                          // 三个顶点的纹理坐标
    uniform_m: Mat4,                                // 模型的变换矩阵m projection*model_view
    uniform_mit: Mat4,                              // m的逆转置矩阵 m.inverse().transpose()
    light_dir: Vec3,
}

impl<'a> PhongShader<'a> {
    pub fn new(
        model: &'a obj::Obj<TexturedVertex, u32>,
        diffuse: &'a ImageBuffer<Rgba<u8>, Vec<u8>>,
        diffuse_nm: &'a ImageBuffer<Rgba<u8>, Vec<u8>>,
        uniform_m: Mat4,
        light_dir: Vec3,
    ) -> Self {
        Self {
            model,
            light_dir,
            diffuse,
            varying_uv: glm::Mat3::one(),
            uniform_m,
            uniform_mit: uniform_m.inverse().unwrap().transpose(),
            diffuse_nm,
        }
    }
}

impl<'a> IShader for PhongShader<'a> {
    fn vertex(&mut self, i_face: usize, nth_vert: usize) -> glm::Vec4 {
        let i_vert = self.model.indices[i_face * 3 + nth_vert];
        let vert = self.model.vertices[i_vert as usize];
        let normal = Vec3::from_array(&vert.normal); // 顶点法向量
        let v = Vec3::from_array(&vert.position); // 顶点位置
        let uv = Vec3::from_array(&vert.texture); // 纹理坐标
        let gl_v = self.uniform_m * v.extend(1.);
        self.varying_uv.as_array_mut()[nth_vert] = uv.clone(); // 每一列是一个顶点出的纹理坐标
        gl_v
    }

    fn fragment(&mut self, bar: glm::Vec3, color: &mut image::Rgba<u8>) -> bool {
        let uv = self.varying_uv * bar; // 用重心坐标插值当前点的纹理坐标
        let px = self.diffuse.get_pixel(
            (uv.x * self.diffuse.width() as f32) as _,
            (uv.y * self.diffuse.height() as f32) as _,
        );
        let nm_px = self.diffuse_nm.get_pixel(
            (uv.x * self.diffuse.width() as f32) as _,
            (uv.y * self.diffuse.height() as f32) as _,
        );

        let mut n = Vec3::from_array(&[nm_px[0] as _, nm_px[1] as _, nm_px[2] as _]).clone(); // 从贴图中加载法向量
        n.as_array_mut()
            .iter_mut()
            .for_each(|v| *v = *v / 255. * 2. - 1.); // tga图像中[0,255], 转换到[-1,-1]

        let n = self.uniform_mit * n.extend(0.); // 法线映射 注意向量转换位齐次坐标是填0
        let n = glm::normalize(vec4_to_3(n)); // 齐次坐标投影回3d 注意向量不需要除w分量

        // let l = self.light_dir.extend(0.); // 之前是在顶点作色器中计算光照，现在要在切空间计算
        // let l = glm::normalize(vec4_to_3(l));
        let l = glm::normalize(self.light_dir); // 全局光照不进行矩阵变换
        let intensity = glm::dot(n, l);

        let r = (px[0] as f32 * intensity) as u8;
        let g = (px[1] as f32 * intensity) as u8;
        let b = (px[2] as f32 * intensity) as u8;
        *color = image::Rgba([r, g, b, 255]);
        return false; // 不丢弃任何像素
    }
}
