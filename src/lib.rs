extern crate ffmpeg_next as ffmpeg;
extern crate image;
extern crate log;
extern crate rscam;

use log::{debug, error, info, trace, warn};
use std::path::PathBuf;

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
    pub fn new_camera(device_: &str, width_: u32, height_: u32, fps_: u32) -> SimpleImageInterface {
        SimpleImageInterface {
            mode: SimpleImageInterfaceMode::Camera,
            camera: Some(Camera::new(device_, width_, height_, fps_)),
            picture: None,
            video: None,
        }
    }

    pub fn new_picture(image_path: impl Into<std::path::PathBuf>) -> SimpleImageInterface {
        SimpleImageInterface {
            mode: SimpleImageInterfaceMode::Picture,
            camera: None,
            picture: Some(Picture::new(image_path.into())),
            video: None,
        }
    }

    pub fn new_video(video_path: impl Into<std::path::PathBuf>) -> SimpleImageInterface {
        SimpleImageInterface {
            mode: SimpleImageInterfaceMode::Video,
            camera: None,
            picture: None,
            video: Some(Video::new(video_path.into())),
        }
    }

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
                interval: (1, fps_), // fps.
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
