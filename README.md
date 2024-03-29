# simple_image_interface

This repository is simple image interface library for rust.
By using this library you can change easily between images, videos, and camera input.
This may be useful for debug like robotics vision area.

- Document
  - [crates.io](https://crates.io/crates/simple_image_interface)
  - [docs.rs](https://docs.rs/simple_image_interface/0.1.0/simple_image_interface/)
- Supported image interface
  - Camera
    - [x] Web Camera (v4l2)
    - [ ] Realsense
    - [ ] Basler Camera
  - Video
    - [x] mp4
  - Image
    - [x] png
    - [x] jpeg

## Get started
### Install

- Install for rscam

```
sudo apt install libv4l-dev
```

- Install for ffmpeg-next

```
sudo apt install -y clang libavcodec-dev libavformat-dev libavutil-dev pkg-config libavdevice-dev
```

- Cargo.toml

```
"simple_image_interface" = "0.1.6"
```

### Example code

- Make interface
  - In detail, [see example code](example/examples.rs)

```rust
use simple_image_interface::simple_image_interface::SimpleImageInterface;

fn main() {
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
}
```

- Example code execution

```sh
# Run for picture
cargo run --release --example example pic

# Run for video
cargo run --release --example example video
```

## Note

- `Simple_image_interface` do not use trait object but lapper struct to improve execution speed

## History

- For new release
  - Updated README
  - Updated Rust edition
- v0.1.6
  - [Updated dependencies and fix some warnings](https://github.com/scepter914/simple-image-interface-rs/pull/2)
  - Refactored
- v0.1.5
  - Refactored
  - \[caution!!\] Changed module architecture
- v0.1.4
  - Renamed repository name
- v0.1.3
  - Refactored
- v0.1.2
  - [Reduced crate size](https://github.com/scepter914/simple-image-interface/pull/1)
- v0.1.1
  - Fixed bug
- v0.1.0
  - Published `simple_image_interface`

## Reference

- [rscam](https://github.com/loyd/rscam): Use for camera input
- [rust-ffmpeg](https://github.com/zmwangx/rust-ffmpeg): Use for video input
