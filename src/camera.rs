use log::info;

/// Camera struct
pub struct Camera {
    camera: rscam::Camera,
    width: u32,
    height: u32,
}

impl Camera {
    /// Web camera interface
    /// # Arguments
    /// - device_: The device name. For example, "/dev/video0"
    /// - width_: The width of camera device
    /// - height_: The height of camera device
    /// - fps_: Frame per seconds
    pub fn new(device_: &str, width_: u32, height_: u32, fps_: u32) -> Camera {
        let mut camera_ = rscam::new(device_).unwrap();
        camera_
            .start(&rscam::Config {
                interval: (1, fps_),
                resolution: (width_, height_),
                format: b"RGB3",
                ..Default::default()
            })
            .unwrap();
        info!("Camera {}: {} * {}, {} fps", device_, width_, height_, fps_);
        Camera {
            camera: camera_,
            width: width_,
            height: height_,
        }
    }

    /// Get frame from interface
    /// If interface do not get a image, return None
    pub fn get_frame(&self) -> Option<image::RgbImage> {
        let frame: rscam::Frame = self.camera.capture().unwrap();
        let rgb_image =
            image::RgbImage::from_vec(self.width, self.height, (&frame[..]).to_vec()).unwrap();
        Some(rgb_image)
    }
}
