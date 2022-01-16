use log::info;

/// Picture struct
pub struct Picture {
    image: image::RgbImage,
    width: u32,
    height: u32,
    is_final_frame: bool,
}

impl Picture {
    pub fn new(image_path: impl Into<std::path::PathBuf>) -> Picture {
        let image_path_into = image_path.into();
        let image_ = image::open(&image_path_into).unwrap().to_rgb8();
        let width_ = image_.width();
        let height_ = image_.height();
        info!("Picture {:?}: {} * {}", image_path_into, width_, height_);
        Picture {
            image: image_,
            width: width_,
            height: height_,
            is_final_frame: false,
        }
    }

    /// Get frame from interface
    /// If interface do not get a image, return None
    pub fn get_frame(&mut self) -> Option<image::RgbImage> {
        let mut output_image = image::RgbImage::new(self.width, self.height);
        output_image.copy_from_slice(&self.image);
        if !self.is_final_frame {
            self.is_final_frame = true;
            Some(output_image)
        } else {
            None
        }
    }
}
