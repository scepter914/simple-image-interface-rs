extern crate ffmpeg_next as ffmpeg;
extern crate image;
extern crate log;
extern crate rscam;

use log::{debug, error, info, trace, warn};

pub struct SimpleImageInterface {
    mode: String,
    camera: Option<Camera>,
    picture: Option<Picture>,
}

impl SimpleImageInterface {
    pub fn new_camera(device_: &str, width_: u32, height_: u32, fps_: u32) -> SimpleImageInterface {
        SimpleImageInterface {
            mode: "Camera".to_string(),
            camera: Some(Camera::new(device_, width_, height_, fps_)),
            picture: None,
        }
    }

    pub fn new_picture(image_path: &str) -> SimpleImageInterface {
        SimpleImageInterface {
            mode: "Picture".to_string(),
            camera: None,
            picture: Some(Picture::new(image_path)),
        }
    }

    pub fn get_frame(&mut self) -> Option<image::RgbImage> {
        match &self.mode[..] {
            "Camera" => self.camera.as_ref().unwrap().get_frame(),
            "Picture" => self.picture.as_mut().unwrap().get_frame(),
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
    pub fn new(path: &std::path::Path) -> Video {
        ffmpeg::init().unwrap();
        let mut ictx_ = ffmpeg::format::input(&path).unwrap();
        let input = ictx_.streams().best(ffmpeg::media::Type::Video).unwrap();
        let video_stream_index = input.index();

        let mut decoder_ = input.codec().decoder().video().unwrap();
        let width_ = decoder_.width();
        let height_ = decoder_.height();

        let mut scaler_ = ffmpeg::software::scaling::context::Context::get(
            decoder_.format(),
            width_,
            height_,
            ffmpeg::format::Pixel::RGB24,
            decoder_.width(),
            decoder_.height(),
            ffmpeg::software::scaling::flag::Flags::BILINEAR,
        )
        .unwrap();

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
        let (stream, packet) = self.ictx.packets().next().unwrap();
        if stream.index() == self.video_stream_index {
            self.decoder.send_packet(&packet);
            let mut decoded = ffmpeg::util::frame::video::Video::empty();
            if self.decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame = ffmpeg::util::frame::video::Video::empty();
                self.scaler.run(&decoded, &mut rgb_frame);

                let rgb_image =
                    image::RgbImage::from_vec(self.width, self.height, rgb_frame.data(0).to_vec())
                        .unwrap();
                Some(rgb_image)
            } else {
                None
            }
        } else {
            None
        }
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
