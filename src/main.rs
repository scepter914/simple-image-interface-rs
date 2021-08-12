extern crate image;
extern crate rscam;

use std::time::Instant;

pub struct Benchmark {
    start_time: Instant,
}

impl Benchmark {
    pub fn set_start_time() -> Benchmark {
        let now_time = Instant::now();
        Benchmark {
            start_time: now_time,
        }
    }

    pub fn print_bench_time(&self) -> () {
        let end = self.start_time.elapsed();
        println!(
            "Process {}.{:03} msec",
            end.as_micros() / 1000,
            end.as_micros() % 1000,
        );
    }
}

fn main() {
    let mut camera = rscam::new("/dev/video0").unwrap();
    camera
        .start(&rscam::Config {
            interval: (1, 50), // fps.
            resolution: (1920, 1080),
            format: b"RGB3",
            ..Default::default()
        })
        .unwrap();

    for i in 0..100 {
        let frame = camera.capture().unwrap();
    }

    println!("capture");
    let bench = Benchmark::set_start_time();
    let frame = camera.capture().unwrap();
    bench.print_bench_time();

    println!("from_raw");
    let bench = Benchmark::set_start_time();
    let rgb_image = image::RgbImage::from_raw(640, 360, (&frame[..]).to_vec()).unwrap();
    bench.print_bench_time();

    rgb_image.save("data/test.png").unwrap();

    println!("from_vec");
    let bench = Benchmark::set_start_time();
    let rgb_image_2 = image::RgbImage::from_vec(640, 360, (&frame[..]).to_vec()).unwrap();
    bench.print_bench_time();

    rgb_image_2.save("data/test_2.png").unwrap();

    // for i in 0..(1920 * 1080 * 3) {
    //     let pixel = buffer[i];
    //     println!("{}: {}", i, pixel);
    // }
    // let mut count = 0;
    // loop {
    //     count += 1;
    //     if 500 <= count {
    //         let mut file = File::create(&format!("data/frame_{:>03}.jpg", count - 500)).unwrap();
    //         file.write_all(&frame[..]).unwrap();
    //     }
    //     if (500 + 500) < count {
    //         break;
    //     }
    // }
}
