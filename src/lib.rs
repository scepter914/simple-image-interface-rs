//! # simple-image-interface-rs
//!
//! - This repository is simple image interface library for rust.
//!   - If you use this library, you can change easily between images, videos, and camera input.
//!   - It may be useful for debug like robotics vision area.
//! - Support image interface
//!   - Camera
//!     - [x] Web Camera (v4l2)
//!     - [ ] Realsense
//!     - [ ] Basler Camera
//!   - Video
//!     - [x] mp4
//!   - Image
//!     - [x] png
//!     - [x] jpeg
//!
//! ## Get started
//!
//! - install for rscam
//!
//! ```
//! sudo apt install libv4l-dev
//! ```
//!
//! - install for ffmpeg-next
//!
//! ```
//! sudo apt install -y clang libavcodec-dev libavformat-dev libavutil-dev pkg-config
//! ```
//!
//! - Cargo.toml
//!
//! ```
//! "simple_image_interface" = "0.1.0"
//! ```
//!
//! - Make interface
//!   - In detail, [See example code](example/examples.rs)
//!
//! ```rust
//!
//!     if args.len() < 2 || &args[1] == "pic" {
//!         interface = SimpleImageInterface::new_picture("./data/from_raw.png");
//!     } else if &args[1] == "video" {
//!         interface = SimpleImageInterface::new_video("./data/random_ball.mp4");
//!     } else {
//!         interface = SimpleImageInterface::new_camera("/dev/video0", 640, 360, 330);
//!         // width, height, fps
//!     }
//!
//!     let mut frame_index = 0;
//!     loop {
//!         frame_index += 1;
//!         let input_image = interface.get_frame();
//!         if input_image.is_none() {
//!             break;
//!         }
//!         my_image_proc(&input_image.unwrap(), frame_index);
//!     }
//! ```
//!
//! ## Note
//!
//! - Not use trait object but lapper struct to improve execution speed
//!
//! ## Reference
//!
//! - <https://github.com/loyd/rscam> : Use for Camera input
//! - <https://github.com/zmwangx/rust-ffmpeg> : Use for Video input
//!
//!

pub mod camera;
pub mod picture;
pub mod video;

use crate::camera::Camera;
use crate::picture::Picture;
use crate::video::Video;

/// Mode enum for SimpleImageInterface
enum SimpleImageInterfaceMode {
    Camera,
    Video,
    Picture,
}

/// SimpleImageInterface struct
pub struct SimpleImageInterface {
    mode: SimpleImageInterfaceMode,
    camera: Option<Camera>,
    picture: Option<Picture>,
    video: Option<Video>,
}

impl SimpleImageInterface {
    /// Init interface from web camera input
    /// # Arguments
    /// - device_: The device name. For example, "/dev/video0"
    /// - width_: The width of camera device
    /// - height_: The height of camera device
    /// - fps_: Frame per seconds
    pub fn new_camera(device_: &str, width_: u32, height_: u32, fps_: u32) -> SimpleImageInterface {
        SimpleImageInterface {
            mode: SimpleImageInterfaceMode::Camera,
            camera: Some(Camera::new(device_, width_, height_, fps_)),
            picture: None,
            video: None,
        }
    }

    /// Init interface from picture (png, jpeg)
    pub fn new_picture(image_path: impl Into<std::path::PathBuf>) -> SimpleImageInterface {
        SimpleImageInterface {
            mode: SimpleImageInterfaceMode::Picture,
            camera: None,
            picture: Some(Picture::new(image_path.into())),
            video: None,
        }
    }

    /// Init interface from video (mp4)
    pub fn new_video(video_path: impl Into<std::path::PathBuf>) -> SimpleImageInterface {
        SimpleImageInterface {
            mode: SimpleImageInterfaceMode::Video,
            camera: None,
            picture: None,
            video: Some(Video::new(video_path.into())),
        }
    }

    /// Get frame from interface
    /// If interface do not get a image, return None
    pub fn get_frame(&mut self) -> Option<image::RgbImage> {
        match self.mode {
            SimpleImageInterfaceMode::Camera => self.camera.as_ref().unwrap().get_frame(),
            SimpleImageInterfaceMode::Picture => self.picture.as_mut().unwrap().get_frame(),
            SimpleImageInterfaceMode::Video => self.video.as_mut().unwrap().get_frame(),
            _ => None,
        }
    }
}
