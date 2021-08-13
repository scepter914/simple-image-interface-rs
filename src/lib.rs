extern crate image;
extern crate log;
extern crate rscam;

use log::{debug, error, info, trace, warn};

pub struct Camera {
    camera: rscam::Camera,
    height: u32,
    width: u32,
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
        return Camera {
            camera: camera_,
            height: height_,
            width: width_,
        };
    }

    pub fn get_frame(&self) -> image::RgbImage {
        let frame: rscam::Frame = self.camera.capture().unwrap();
        let rgb_image =
            image::RgbImage::from_vec(self.width, self.height, (&frame[..]).to_vec()).unwrap();
        return rgb_image;
    }
}
