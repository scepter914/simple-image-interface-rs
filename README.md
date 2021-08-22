# simple-image-interface

- This repository is simple image interface library for rust
- Support image interface
  - [ x ] Camera input (v4l2)
  - [ ] Realsense
  - [ ] Basler Camera
  - [ ] video (mp4)
  - [ x ] image (png, jpg)

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
- example

## Note

- use not trait object but lapper struct to improve execution speed

## Reference

- <https://github.com/loyd/rscam> : Use for Camera input
