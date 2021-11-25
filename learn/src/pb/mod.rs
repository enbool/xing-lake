mod abi;

use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use photon_rs::transform::{resize, SamplingFilter};
pub use crate::pb::abi::*;
use crate::pb::abi::resize::SampleFilter;

// 构造方法
impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
    pub fn decode(data: &[u8]) -> Self{
        Self{ specs: vec![]}
    }
}

// ImageSpec to base64 string
impl From<&ImageSpec> for String {
    fn from(image_spec: &ImageSpec) -> Self {
        let data = image_spec.encode_to_vec();
        encode_config(data, URL_SAFE_NO_PAD)
    }
}

// 字符串转 ImageSpec
impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = decode_config(value, URL_SAFE_NO_PAD)?;
        Ok(ImageSpec::decode(&data[..])?)
    }
}

// filter to string
impl filter::Filter {
    pub fn to_string(&self) -> Option<&'static str> {
        match self {
            filter::Filter::Unspecified => None,
            filter::Filter::Oceanic => Some("oceanic"),
            filter::Filter::Islands => Some("islands"),
            filter::Filter::Marine => Some("marine"),
        }
    }
}

// 自定义的 SampleFilter 转换到 photon_rs 库的 SamplingFilter
impl From<resize::SampleFilter> for SamplingFilter {
    fn from(sampleFilter: SampleFilter) -> Self {
        match sampleFilter {
            SampleFilter::Undefined => SamplingFilter::Nearest,
            SampleFilter::Nearest => SamplingFilter::Nearest,
            SampleFilter::Triangle => SamplingFilter::Triangle,
            SampleFilter::CatmullRom => SamplingFilter::CatmullRom,
            SampleFilter::Gaussian => SamplingFilter::Gaussian,
            SampleFilter::Lanczos3 => SamplingFilter::Lanczos3,
        }
    }
}

// 快捷方法
impl Spec {
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::SeamCarve as i32,
                filter: resize::SampleFilter::Undefined as i32,
            }))
        }
    }

    pub fn new_resize(width: u32, height: u32, filter: resize::SampleFilter) -> Self
    {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::Normal as i32,
                filter: filter as i32,
            }))
        }
    }
    pub fn new_filter(filter: filter::Filter) -> Self { Self { data: Some(spec::Data::Filter(Filter { filter: filter as i32 })) } }
    pub fn new_watermark(x: u32, y: u32) -> Self {
        Self { data: Some(spec::Data::Watermark(WaterMark { x, y })) } }
}

#[cfg(test)]
mod tests{
    use std::borrow::Borrow;
    use super::*;

    #[cfg(test)]
    fn encode_decode_test(){
        let spec1 = Spec::new_resize(600, 600, resize::SampleFilter::CatmullRom);
        let spec2 = Spec::new_filter(filter::Filter::Marine);
        let image_spec = ImageSpec::new(vec![spec1, spec2]);
        let encoded: String = image_spec.borrow().into();
        assert_eq!(image_spec, encoded.as_str().try_into().unwrap())
    }
}