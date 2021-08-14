use std::env;

use simple_image_interface::SimpleImageInterface;

fn my_image_proc(rgb_image: &image::RgbImage) -> () {
    let width = rgb_image.width();
    let height = rgb_image.height();
    let mut gray_image = image::GrayImage::new(width, height);
    // for example gray scale
    for i in 0..width {
        for j in 0..height {
            let pixel = rgb_image.get_pixel(i, j);
            let gray_pixel = [((pixel[0] as f32 * 0.2126) as u32
                + (pixel[1] as f32 * 0.7152) as u32
                + (pixel[2] as f32 * 0.0722) as u32) as u8; 1];
            gray_image.put_pixel(i, j, image::Luma(gray_pixel));
        }
    }
    println!("save gray scale image");
    gray_image.save("data/sample.png").unwrap();
}

fn main() {
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Warn,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    let args: Vec<String> = env::args().collect();
    let mut interface: SimpleImageInterface;

    if args.len() < 2 || &args[1] == "pic" {
        interface = SimpleImageInterface::new_picture("data/from_raw.png");
    } else if &args[1] == "video" {
        interface = SimpleImageInterface::new_camera("/dev/video0", 640, 360, 330);
    } else {
        interface = SimpleImageInterface::new_camera("/dev/video0", 640, 360, 330);
    }

    loop {
        let input_image = interface.get_frame();
        if input_image.is_none() {
            break;
        }
        my_image_proc(&input_image.unwrap());
    }
}
