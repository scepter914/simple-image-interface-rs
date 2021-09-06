# simple-image-interface-rs

- This repository is simple image interface library for rust.
  - If you use this library, you can change easily between images, videos, and camera input.
  - It may be useful for debug like robotics vision area.
- Support image interface
  - Camera
    - [x] Web Camera (v4l2)
    - [ ] Realsense
    - [ ] Basler Camera
  - Video
    - [x] mp4
  - Image
    - [x] png
    - [x] jpeg

## Document

- [crates.io](https://crates.io/crates/simple_image_interface)
- [docs.rs](https://docs.rs/simple_image_interface/0.1.0/simple_image_interface/)

## Get started

- install for rscam

```
sudo apt install libv4l-dev
```

- install for ffmpeg-next

```
sudo apt install -y clang libavcodec-dev libavformat-dev libavutil-dev pkg-config
```

- Cargo.toml

```
"simple_image_interface" = "0.1.2"
```

- Make interface
  - In detail, [See example code](example/examples.rs)

```rust

    if args.len() < 2 || &args[1] == "pic" {
        interface = SimpleImageInterface::new_picture("./data/from_raw.png");
    } else if &args[1] == "video" {
        interface = SimpleImageInterface::new_video("./data/random_ball.mp4");
    } else {
        interface = SimpleImageInterface::new_camera("/dev/video0", 640, 360, 330);
        // width, height, fps
    }

    let mut frame_index = 0;
    loop {
        frame_index += 1;
        let input_image = interface.get_frame();
        if input_image.is_none() {
            break;
        }
        my_image_proc(&input_image.unwrap(), frame_index);
    }
```

## Note

- Not use trait object but lapper struct to improve execution speed

## History

- v0.1.2
  - [Reduce crate size ](https://github.com/scepter914/simple-image-interface-rs/pull/1)
- v0.1.1
  - fix bug
- v0.1.0
  - publish initial library

## Reference

- <https://github.com/loyd/rscam> : Use for Camera input
- <https://github.com/zmwangx/rust-ffmpeg> : Use for Video input
