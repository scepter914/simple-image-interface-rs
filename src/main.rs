extern crate rscam;

use std::fs::File;
use std::io::Write;

fn main() {
    let mut camera = rscam::new("/dev/video0").unwrap();

    camera
        .start(&rscam::Config {
            interval: (1, 330), // fps.
            resolution: (640, 360),
            format: b"MJPG",
            ..Default::default()
        })
        .unwrap();
    let mut count = 0;
    loop {
        count += 1;
        let frame = camera.capture().unwrap();
        if 500 <= count {
            let mut file = File::create(&format!("data/frame_{:>03}.jpg", count - 500)).unwrap();
            file.write_all(&frame[..]).unwrap();
        }
        if (500 + 500) < count {
            break;
        }
    }
}
