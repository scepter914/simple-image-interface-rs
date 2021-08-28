# simple-image-interface

- This repository is simple image interface library for rust
- Support image interface
  - Camera
    - [ x ] Web Camera (v4l2)
    - [ ] Realsense
    - [ ] Basler Camera
  - Video
    - [ x ] mp4
  - Image
    - [ x ] png
    - [ x ] jpeg

## Get started (Under construction)

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
"simple_image_interface" = "0.1.0"
```

- And [See example code](example/examples.rs)

## Note

- Not use trait object but lapper struct to improve execution speed

## Reference

- <https://github.com/loyd/rscam> : Use for Camera input
