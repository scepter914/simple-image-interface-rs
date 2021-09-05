use simple_image_interface::SimpleImageInterface;
use std::env;

#[test]
fn is_able_to_get_all_frames_from_mp4() {
    let mut interface = SimpleImageInterface::new_video("./data/random_ball.mp4");
    let mut frame_index = 0;
    loop {
        frame_index += 1;
        let input_image = interface.get_frame();
        if input_image.is_none() {
            frame_index -= 1;
            break;
        }
    }
    assert_eq!(751, frame_index);
}

#[test]
fn is_valid_frame_from_mp4() {
    let mut interface = SimpleImageInterface::new_video("./data/random_ball.mp4");
    let mut frame_index = 0;
    let mut input_image;
    loop {
        frame_index += 1;
        input_image = interface.get_frame();
        if frame_index == 200 {
            break;
        }
        if input_image.is_none() {
            frame_index -= 1;
            break;
        }
        assert!(!input_image.is_none(), "invalid frame is reading");
    }
    let path = "./data/sample_200.png";
    let answer_img = image::open(path).unwrap().to_rgb8();
    assert_eq!(input_image.unwrap(), answer_img);
}

#[test]
fn is_able_to_get_all_frames_from_picture() {
    let mut interface = SimpleImageInterface::new_picture("./data/from_raw.png");
    let mut frame_index = 0;
    loop {
        frame_index += 1;
        let input_image = interface.get_frame();

        if input_image.is_none() {
            frame_index -= 1;
            break;
        }
    }
    assert_eq!(1, frame_index);
}

#[test]
fn is_valid_frame_from_picture() {
    let mut interface = SimpleImageInterface::new_picture("./data/from_raw.png");
    let mut frame_index = 0;
    let mut input_image;
    loop {
        frame_index += 1;
        input_image = interface.get_frame();
        if frame_index == 1 {
            break;
        }
        if input_image.is_none() {
            frame_index -= 1;
            break;
        }
    }
    let path = "./data/from_raw.png";
    let answer_img = image::open(path).unwrap().to_rgb8();
    assert_eq!(input_image.unwrap(), answer_img);
}
