extern crate log;
use log::{debug, error, info, trace, warn};

pub struct Camera {
    camera: rscam::Camera,
}

impl Camera {
    pub fn new(device_: &str, fps_: u32, resolution_: (u32, u32)) -> Camera {
        let mut camera_ = rscam::new(device_).unwrap();
        camera_
            .start(&rscam::Config {
                interval: (1, fps_), // fps.
                resolution: resolution_,
                format: b"MJPG",
                ..Default::default()
            })
            .unwrap();
        return Camera { camera: camera_ };
    }

    pub fn get_frame(&self) -> rscam::Frame {
        let frame: rscam::Frame = self.camera.capture().unwrap();
        return frame;
    }
}
