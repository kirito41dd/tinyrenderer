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
    diffuse_spec: &'a ImageBuffer<Rgba<u8>, Vec<u8>>, // 高光贴图
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
        diffuse_spec: &'a ImageBuffer<Rgba<u8>, Vec<u8>>,
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
            diffuse_spec,
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
            (uv.x * self.diffuse_nm.width() as f32) as _,
            (uv.y * self.diffuse_nm.height() as f32) as _,
        );
        let spec_px = self.diffuse_spec.get_pixel(
            (uv.x * self.diffuse_spec.width() as f32) as _,
            (uv.y * self.diffuse_spec.height() as f32) as _,
        );
        let spec_v = spec_px[0] as f32 / 1.; // 光泽值, 这个值越小越反射范围越大，越不光泽，越大越有光泽

        let mut n = Vec3::from_array(&[nm_px[0] as _, nm_px[1] as _, nm_px[2] as _]).clone(); // 从贴图中加载法向量
        n.as_array_mut()
            .iter_mut()
            .for_each(|v| *v = *v / 255. * 2. - 1.); // tga图像中[0,255], 转换到[-1,-1]

        let n = self.uniform_mit * n.extend(0.); // 法线映射 注意向量转换位齐次坐标是填0
        let n = glm::normalize(vec4_to_3(n)); // 齐次坐标投影回3d 注意向量不需要除w分量

        let l = self.uniform_m * self.light_dir.extend(0.); // 映射光照方向
        let l = glm::normalize(vec4_to_3(l));

        let r = glm::normalize(n * (glm::dot(n, l) * 2.) - l); // 反射光方向

        let spec = glm::pow(r.z.max(0.), spec_v); // 我们从z轴看, dot(v,r)
        let diff = glm::dot(n, l).max(0.);

        let arg_ambient = 5.; // 环境光
        let arg_diffuse = 1.; // 漫反射光
        let arg_specular = 0.6; // 镜面反射光
        let intensity = glm::dot(n, l);

        let r = (arg_ambient + px[0] as f32 * (arg_diffuse * diff + arg_specular * spec)) as u8;
        let g = (arg_ambient + px[1] as f32 * (arg_diffuse * diff + arg_specular * spec)) as u8;
        let b = (arg_ambient + px[2] as f32 * (arg_diffuse * diff + arg_specular * spec)) as u8;
        *color = image::Rgba([r, g, b, 255]);
        return false; // 不丢弃任何像素
    }
}
