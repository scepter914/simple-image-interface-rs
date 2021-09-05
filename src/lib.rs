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

extern crate ffmpeg_next as ffmpeg;
extern crate image;
extern crate log;
extern crate rscam;

use log::{debug, error, info, trace, warn};

enum SimpleImageInterfaceMode {
    Camera,
    Video,
    Picture,
}

pub struct SimpleImageInterface {
    mode: SimpleImageInterfaceMode,
    camera: Option<Camera>,
    picture: Option<Picture>,
    video: Option<Video>,
}

impl SimpleImageInterface {
    /// - Init interface from web camera input
    pub fn new_camera(device_: &str, width_: u32, height_: u32, fps_: u32) -> SimpleImageInterface {
        SimpleImageInterface {
            mode: SimpleImageInterfaceMode::Camera,
            camera: Some(Camera::new(device_, width_, height_, fps_)),
            picture: None,
            video: None,
        }
    }

    /// - Init interface from picture (png, jpeg)
    pub fn new_picture(image_path: impl Into<std::path::PathBuf>) -> SimpleImageInterface {
        SimpleImageInterface {
            mode: SimpleImageInterfaceMode::Picture,
            camera: None,
            picture: Some(Picture::new(image_path.into())),
            video: None,
        }
    }

    /// - Init interface from video (mp4)
    pub fn new_video(video_path: impl Into<std::path::PathBuf>) -> SimpleImageInterface {
        SimpleImageInterface {
            mode: SimpleImageInterfaceMode::Video,
            camera: None,
            picture: None,
            video: Some(Video::new(video_path.into())),
        }
    }

    /// - get frame from interface
    /// - If interface do not get a image, return None
    pub fn get_frame(&mut self) -> Option<image::RgbImage> {
        match self.mode {
            SimpleImageInterfaceMode::Camera => self.camera.as_ref().unwrap().get_frame(),
            SimpleImageInterfaceMode::Picture => self.picture.as_mut().unwrap().get_frame(),
            SimpleImageInterfaceMode::Video => self.video.as_mut().unwrap().get_frame(),
            _ => None,
        }
    }
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

    pub fn get_frame(&self) -> Option<image::RgbImage> {
        let frame: rscam::Frame = self.camera.capture().unwrap();
        let rgb_image =
            image::RgbImage::from_vec(self.width, self.height, (&frame[..]).to_vec()).unwrap();
        Some(rgb_image)
    }
}

pub struct Video {
    decoder: ffmpeg::decoder::Video,
    ictx: ffmpeg::format::context::Input,
    scaler: ffmpeg::software::scaling::Context,
    width: u32,
    height: u32,
    video_stream_index: usize,
}

impl Video {
    pub fn new(path: impl Into<std::path::PathBuf>) -> Video {
        ffmpeg::init().unwrap();
        let path_into = path.into();

        let ictx_ = ffmpeg::format::input(&path_into).unwrap();
        let input = ictx_.streams().best(ffmpeg::media::Type::Video).unwrap();
        let video_stream_index = input.index();

        let decoder_ = input.codec().decoder().video().unwrap();
        let width_ = decoder_.width();
        let height_ = decoder_.height();

        let scaler_ = ffmpeg::software::scaling::context::Context::get(
            decoder_.format(),
            width_,
            height_,
            ffmpeg::format::Pixel::RGB24,
            width_,
            height_,
            ffmpeg::software::scaling::flag::Flags::BILINEAR,
        )
        .unwrap();

        info!("Video {:?}: {} * {}", &path_into, width_, height_);
        Video {
            decoder: decoder_,
            ictx: ictx_,
            scaler: scaler_,
            width: width_,
            height: height_,
            video_stream_index,
        }
    }

    pub fn get_frame(&mut self) -> Option<image::RgbImage> {
        let mut is_valid_frame = false;
        while !is_valid_frame {
            let stream_and_packet_iter = self.ictx.packets().next();
            if stream_and_packet_iter.is_some() {
                let (stream, packet) = stream_and_packet_iter.unwrap();
                if stream.index() == self.video_stream_index {
                    self.decoder.send_packet(&packet).unwrap();
                    let mut decoded = ffmpeg::util::frame::video::Video::empty();
                    if self.decoder.receive_frame(&mut decoded).is_ok() {
                        let mut rgb_frame = ffmpeg::util::frame::video::Video::empty();
                        self.scaler.run(&decoded, &mut rgb_frame).unwrap();

                        let rgb_image = image::RgbImage::from_vec(
                            self.width,
                            self.height,
                            rgb_frame.data(0).to_vec(),
                        )
                        .unwrap();
                        is_valid_frame = true;
                        return Some(rgb_image);
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        return None;
    }
}

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
