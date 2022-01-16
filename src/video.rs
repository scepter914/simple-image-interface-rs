use ffmpeg_next as ffmpeg;
use log::info;

/// Video struct
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

    /// Get frame from interface
    /// If interface do not get a image, return None
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
