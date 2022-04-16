use crate::pb::Spec;
use image::ImageOutputFormat;
mod photon;
pub use photon::Photon;

pub trait Engine {
    // 应用接口
    fn apply(&mut self, specs: &[Spec]);

    // 生成接口
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

// 表达式转换接口
pub trait SpecTransform<T> {
    // 将类型转换成可识别的Spec
    fn transform(&mut self, op: T);
}
