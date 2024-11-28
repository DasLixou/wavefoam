//! This is copied and modified from rust-skia/rust-skia's hello example, which can be found [here](https://github.com/rust-skia/rust-skia/blob/c5faa8f99ea9cc4a0235e8aa8c28c970e301a436/skia-safe/examples/hello/canvas.rs)

use std::mem;

use skia_safe::{surfaces, Color, Data, EncodedImageFormat, Paint, PaintStyle, Path, Surface};

pub struct Canvas {
    pub surface: Surface,
    pub paint: Paint,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Canvas {
        let mut surface = surfaces::raster_n32_premul((width, height)).expect("surface");
        let mut paint = Paint::default();
        paint.set_color(Color::BLACK);
        paint.set_anti_alias(true);
        paint.set_stroke_width(1.0);
        surface.canvas().clear(Color::WHITE);
        Canvas { surface, paint }
    }

    #[inline]
    pub fn save(&mut self) {
        self.canvas().save();
    }

    #[inline]
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.canvas().translate((dx, dy));
    }

    #[inline]
    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.canvas().scale((sx, sy));
    }

    #[inline]
    pub fn data(&mut self) -> Data {
        let image = self.surface.image_snapshot();
        let mut context = self.surface.direct_context();
        image
            .encode(context.as_mut(), EncodedImageFormat::PNG, None)
            .unwrap()
    }

    #[inline]
    fn canvas(&mut self) -> &skia_safe::Canvas {
        self.surface.canvas()
    }
}
