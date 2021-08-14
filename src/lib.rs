extern crate image;
extern crate log;
extern crate rscam;

use log::{debug, error, info, trace, warn};

pub trait ImageInterface {
    fn get_frame(&mut self) -> Option<image::RgbImage>;
}

pub struct Camera {
    camera: rscam::Camera,
    width: u32,
    height: u32,
}

impl Camera {
    pub fn new(device_: &str, width_: u32, height_: u32, fps_: u32) -> Camera {
        let mut camera_ = rscam::new(device_).unwrap();
        camera_
            .start(&rscam::Config {
                interval: (1, fps_), // fps.
                resolution: (width_, height_),
                format: b"RGB3",
                ..Default::default()
            })
            .unwrap();
        info!("Camera {}: {} * {}, {} fps", device_, width_, height_, fps_);
        return Camera {
            camera: camera_,
            width: width_,
            height: height_,
        };
    }
}

impl ImageInterface for Camera {
    fn get_frame(&mut self) -> Option<image::RgbImage> {
        let frame: rscam::Frame = self.camera.capture().unwrap();
        let rgb_image =
            image::RgbImage::from_vec(self.width, self.height, (&frame[..]).to_vec()).unwrap();
        return Some(rgb_image);
    }
}

pub struct Picture {
    image: image::RgbImage,
    width: u32,
    height: u32,
    is_final_frame: bool,
}

impl Picture {
    pub fn new(image_path: &str) -> Picture {
        let image_ = image::open(image_path).unwrap().to_rgb8();
        let width_ = image_.width();
        let height_ = image_.height();
        info!("Picture {}: {} * {}", image_path, width_, height_);
        return Picture {
            image: image_,
            width: width_,
            height: height_,
            is_final_frame: false,
        };
    }
}

impl ImageInterface for Picture {
    fn get_frame(&mut self) -> Option<image::RgbImage> {
        let mut output_image = image::RgbImage::new(self.width, self.height);
        output_image.copy_from_slice(&self.image);
        if !self.is_final_frame {
            self.is_final_frame = true;
            return Some(output_image);
        } else {
            return None;
        }
    }
}
