use std::{fs::File, io::Write};

use canvas::Canvas;
use hound::{SampleFormat, WavReader};
use skia_safe::{Rect, V4};
use wavefoam::scan::PeakScan;

mod canvas;

const WIDTH: i32 = 2560;
const HEIGHT: i32 = 1280;

fn main() {
    let sample = include_bytes!("guitar.wav");
    let wav_reader = WavReader::new(sample.as_slice()).unwrap();
    assert_eq!(wav_reader.spec().sample_format, SampleFormat::Float);
    let peak_scan = PeakScan::from_iter(wav_reader.into_samples::<f32>().map(Result::unwrap), 256);
    let peak_image = wavefoam_skia::make_scan_image(&peak_scan).unwrap();

    let mut canvas = Canvas::new(peak_image.width(), peak_image.width() / 10);

    let rtef = wavefoam_skia::make_runtime_effect().unwrap();

    let shader =
        wavefoam_skia::make_shader(&rtef, &peak_image, V4::new(1.0, 0.0, 0.5, 1.0)).unwrap();

    canvas.scale(1.0, peak_image.width() as f32 / 10.0);
    canvas.paint.set_shader(shader);
    canvas.surface.canvas().draw_rect(
        Rect::from_iwh(peak_image.width(), 1),
        &mut canvas.paint,
    );

    let d = canvas.data();
    let mut file = File::create("test.png").unwrap();
    let bytes = d.as_bytes();
    file.write_all(bytes).unwrap();
}
