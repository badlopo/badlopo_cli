use clap::ValueEnum;
use std::path::PathBuf;

#[derive(ValueEnum, Clone, Debug)]
pub enum ImageFormat {
    // TODO: add supported format
}

#[derive(Clone, Debug)]
pub enum ImageSize {
    /// Both width and height are specified
    Both(u32, u32),
    /// Only width is specified
    Width(u32),
    /// Only height is specified
    Height(u32),
    /// Both width and height are omitted
    None,
}

impl From<&str> for ImageSize {
    fn from(value: &str) -> Self {
        if let [w, h] = value.split("x").collect::<Vec<&str>>()[..] {
            let w = w.parse::<u32>();
            let h = h.parse::<u32>();

            match (w, h) {
                (Ok(w), Ok(h)) => ImageSize::Both(w, h),
                (Ok(w), Err(_)) => ImageSize::Width(w),
                (Err(_), Ok(h)) => ImageSize::Height(h),
                (Err(_), Err(_)) => ImageSize::None,
            }
        } else {
            ImageSize::None
        }
    }
}

struct ImageImpl;

impl ImageImpl {
    pub fn handle(source: PathBuf, format: Option<ImageFormat>, size: ImageSize) {
        todo!()
    }
}
