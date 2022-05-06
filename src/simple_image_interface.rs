use crate::interface::camera::Camera;
use crate::interface::picture::Picture;
use crate::interface::video::Video;

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
