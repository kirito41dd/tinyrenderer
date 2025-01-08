use glm::Vec3;

pub mod shader_impl_gouraud_shader;

pub trait IShader {
    /// 顶点着色器
    ///
    /// iface 第i个面, nth_vert 面的第n个顶点
    ///
    /// 返回顶点在裁剪空间的坐标(齐次坐标)
    fn vertex(&mut self, i_face: usize, nth_vert: usize) -> glm::Vec4;
    /// 片段着色器
    ///
    /// bar 当前像素在三角形中的重心坐标 color 像素颜色
    ///
    /// 返回true表示丢弃当前像素
    fn fragment(&mut self, bar: Vec3, color: &mut image::Rgba<u8>) -> bool;
}
