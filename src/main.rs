use std::env;

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
    let interface;

    if args.len() < 2 {
        interface = simple_image_interface::Camera::new("/dev/video0", 640, 360, 330);
        //interface = simple_image_interface::Picture::new("data/from_raw.png");
    } else if &args[1] == "video" {
        interface = simple_image_interface::Camera::new("/dev/video0", 640, 360, 330);
        //interface = simple_image_interface::Picture::new("data/from_raw.png");
    } else if &args[1] == "pic" {
        //interface = simple_image_interface::Picture::new("data/from_raw.png");
        interface = simple_image_interface::Camera::new("/dev/video0", 640, 360, 330);
    } else {
        //interface = simple_image_interface::Picture::new("data/from_raw.png");
        interface = simple_image_interface::Camera::new("/dev/video0", 640, 360, 330);
    }

    loop {
        let input_image = interface.get_frame();
        if input_image.is_none() {
            break;
        }
        my_image_proc(&input_image.unwrap());
    }
}
