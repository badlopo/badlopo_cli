use clap::ValueEnum;
use image::imageops::FilterType;
use image::{
    guess_format, load_from_memory_with_format, ColorType, DynamicImage, ImageFormat as ImgFormat,
};
use std::fmt::{Debug, Formatter};
use std::fs::{read, File};
use std::path::PathBuf;

/// Supported target image format.
#[derive(ValueEnum, Clone, Debug, Eq, PartialEq)]
pub enum ImageFormat {
    Png,
    /// alias for JPEG
    Jpg,
    Jpeg,
    Gif,
    WebP,
    Pnm,
    Tiff,
    Tga,
    Bmp,
    Ico,
    Hdr,
    /// alias for OpenEXR
    Exr,
    OpenEXR,
    /// alias for Farbfeld
    FF,
    Farbfeld,
    Qoi,
    Pcx,
}

impl TryFrom<ImgFormat> for ImageFormat {
    type Error = ();

    fn try_from(value: ImgFormat) -> Result<Self, Self::Error> {
        match value {
            ImgFormat::Png => Ok(ImageFormat::Png),
            ImgFormat::Jpeg => Ok(ImageFormat::Jpeg),
            ImgFormat::Gif => Ok(ImageFormat::Gif),
            ImgFormat::Pnm => Ok(ImageFormat::Pnm),
            ImgFormat::Tiff => Ok(ImageFormat::Tiff),
            ImgFormat::Tga => Ok(ImageFormat::Tga),
            ImgFormat::Bmp => Ok(ImageFormat::Bmp),
            ImgFormat::Ico => Ok(ImageFormat::Ico),
            ImgFormat::Hdr => Ok(ImageFormat::Hdr),
            ImgFormat::OpenExr => Ok(ImageFormat::OpenEXR),
            ImgFormat::Farbfeld => Ok(ImageFormat::Farbfeld),
            ImgFormat::Qoi => Ok(ImageFormat::Qoi),
            ImgFormat::Pcx => Ok(ImageFormat::Pcx),
            ImgFormat::WebP => Ok(ImageFormat::WebP),
            ImgFormat::Dds => Err(()),
            ImgFormat::Avif => Err(()),
            _ => Err(()),
        }
    }
}

impl From<ImageFormat> for ImgFormat {
    fn from(value: ImageFormat) -> Self {
        match value {
            ImageFormat::Png => ImgFormat::Png,
            ImageFormat::Jpg | ImageFormat::Jpeg => ImgFormat::Jpeg,
            ImageFormat::Gif => ImgFormat::Gif,
            ImageFormat::WebP => ImgFormat::WebP,
            ImageFormat::Pnm => ImgFormat::Pnm,
            ImageFormat::Tiff => ImgFormat::Tiff,
            ImageFormat::Tga => ImgFormat::Tga,
            ImageFormat::Bmp => ImgFormat::Bmp,
            ImageFormat::Ico => ImgFormat::Ico,
            ImageFormat::Hdr => ImgFormat::Hdr,
            ImageFormat::Exr | ImageFormat::OpenEXR => ImgFormat::OpenExr,
            ImageFormat::FF | ImageFormat::Farbfeld => ImgFormat::Farbfeld,
            ImageFormat::Qoi => ImgFormat::Qoi,
            ImageFormat::Pcx => ImgFormat::Pcx,
        }
    }
}

/// Corresponding struct for the size argument.
#[derive(Clone, Debug, Eq, PartialEq)]
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

impl ImageSize {
    pub fn to_wh(self, width: u32, height: u32) -> Option<(u32, u32)> {
        let aspect_ratio = width as f32 / height as f32;
        match self {
            ImageSize::Both(w, h) => Some((w, h)),
            ImageSize::Width(w) => Some((w, (w as f32 / aspect_ratio) as u32)),
            ImageSize::Height(h) => Some(((h as f32 * aspect_ratio) as u32, h)),
            ImageSize::None => None,
        }
    }
}

struct ImageWithMeta {
    format: ImageFormat,
    width: u32,
    height: u32,
    color_type: ColorType,
    bit_depth: u8,
    dynamic_image: DynamicImage,
}

/// override the Debug trait for ImageWithMeta to hide the 'dynamic_image' field (too verbose)
impl Debug for ImageWithMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageWithMeta")
            .field("format", &self.format)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("color_type", &self.color_type)
            .field("bit_depth", &self.bit_depth)
            .finish()
    }
}

impl ImageWithMeta {
    /// show the metadata of the image in console
    pub fn show_meta(&self) {
        println!(
            "===== =====\nFormat: {:?}\nWidth: {}\nHeight: {}\nColor Type: {:?}\nBit Depth: {}",
            self.format, self.width, self.height, self.color_type, self.bit_depth
        );
    }
}

pub struct ImageImpl;

impl ImageImpl {
    /// parse the image buffer and return the dynamic image with its metadata
    fn parse(buffer: &[u8]) -> Result<ImageWithMeta, String> {
        if let Ok(f) = guess_format(buffer) {
            if let Ok(format) = ImageFormat::try_from(f) {
                match load_from_memory_with_format(buffer, f) {
                    Ok(dynamic_image) => Ok(ImageWithMeta {
                        format,
                        width: dynamic_image.width(),
                        height: dynamic_image.height(),
                        color_type: dynamic_image.color(),
                        bit_depth: dynamic_image.color().bytes_per_pixel(),
                        dynamic_image,
                    }),
                    Err(err) => Err(format!("{}", err)),
                }
            } else {
                Err("unsupported format".to_string())
            }
        } else {
            Err("unknown format".to_string())
        }
    }

    /// 1. show the metadata of the image (always do)
    /// 2. resize the image if the size is specified
    /// 3. convert the image to the target format if specified
    /// 4. write the image to the target path if at least one of the 'size' and 'format' is specified
    pub fn handle(source: PathBuf, format: Option<ImageFormat>, size: Option<ImageSize>) {
        if source.is_file() {
            if let Ok(buffer) = read(&source) {
                match ImageImpl::parse(&buffer) {
                    Ok(parsed) => {
                        // always show metadata of the image
                        parsed.show_meta();

                        // check if the size or format is specified
                        if format.is_some() || {
                            match &size {
                                Some(v) => v != &ImageSize::None,
                                None => false,
                            }
                        } {
                            let target_format = ImgFormat::from(format.unwrap_or(parsed.format));

                            // we assume that the conversion is successful
                            let _stem = source.file_stem().unwrap().to_str().unwrap().to_string();
                            let _extension = target_format.extensions_str()[0];

                            let mut _path = source.clone();
                            let img = if let Some((w, h)) = {
                                match size {
                                    Some(size) => size.to_wh(parsed.width, parsed.height),
                                    None => None,
                                }
                            } {
                                // patch 'file_stem' with '@<width>x<height>'
                                _path = _path.with_file_name(format!("{}@{}x{}", _stem, w, h,));

                                // resize the image
                                &parsed.dynamic_image.resize_exact(w, h, FilterType::Nearest)
                            } else {
                                // otherwise, use the original image
                                &parsed.dynamic_image
                            };

                            // set 'extension'
                            _path = _path.with_extension(_extension);

                            println!("===== =====");
                            // try to open a file in write-only mode at the target path
                            match File::create(&_path) {
                                Ok(mut target) => match img.write_to(&mut target, target_format) {
                                    Ok(_) => println!("Image saved to {:?}", _path),
                                    Err(err) => println!("Error: {}", err),
                                },
                                Err(err) => println!("Error: {}", err),
                            }
                        }
                    }
                    Err(err) => println!("Error: {}", err),
                }
            } else {
                println!("Error: failed to read the source file.");
            }
        } else {
            println!("Invalid source. (not a file)");
        }
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        // let bytes = include_bytes!("../__test__/img.png");
        // let dyn_image = load_from_memory(bytes).unwrap();
        //
        // let target = PathBuf::from("./a.png");
        // let mut target = File::create(target).unwrap();
        // dyn_image.write_to(&mut target, ImgFormat::Png).unwrap();

        // let bytes = vec![10, 10, 10, 10, 10, 10];
        // let bytes = vec![0x88, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

        // println!("{:#?}", guess_format(&bytes));
        ImageImpl::handle(
            PathBuf::from("./__test__/24.png"),
            Some(ImageFormat::Tiff),
            Some(ImageSize::Width(256)),
        );

        // let file = PathBuf::from("./__test__/img.png");
        // let file = file.with_extension("@3x3.png");
        // println!("{:?}", file.file_stem());
    }
}
