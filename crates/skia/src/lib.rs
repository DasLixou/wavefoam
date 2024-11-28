use skia_safe::{
    images, AlphaType, ColorType, Data, Image, ImageInfo, RuntimeEffect, SamplingOptions, Shader,
    V4,
};
use wavefoam::scan::PeakScan;

pub fn make_runtime_effect() -> Result<RuntimeEffect, String> {
    RuntimeEffect::make_for_shader(
        r#"
    layout(color)   uniform vec4 color;
                    uniform shader peaks;
    
    half4 main(float2 coord) {
        float min_peak = peaks.eval(float2(coord.x, 0)).x;
        float max_peak = peaks.eval(float2(coord.x, 2)).x;
        
        float v = coord.y;
        return (v < max_peak && v > min_peak) ? color.rgb1 : half4(0.0, 0.0, 0.0, 0.0);
    }
    "#,
        None,
    )
}

pub fn make_shader(effect: &RuntimeEffect, peak_image: &Image, color: V4) -> Option<Shader> {
    let uniforms = Data::new_zero_initialized(effect.uniform_size());
    let offset = effect.find_uniform("color").unwrap().offset();
    unsafe {
        let slice = &uniforms.as_bytes()[offset..];
        std::ptr::copy_nonoverlapping::<V4>(&color, slice.as_ptr() as *mut _, 1);
    }
    let image_shader = peak_image
        .to_shader(None, SamplingOptions::default(), None)
        .unwrap();
    effect.make_shader(uniforms, &[image_shader.into()], None)
}

pub fn make_scan_image(scan: &PeakScan) -> Option<Image> {
    let mut data = vec![0u8; scan.resolution() * 2];
    for (i, peak) in scan.peaks().iter().enumerate() {
        let min: u8 = ((peak.min + 1.0) * 128.) as u8;
        let max: u8 = ((peak.max + 1.0) * 128.) as u8;
        data[i] = min;
        data[i + scan.resolution()] = max;
    }
    let data = Data::new_copy(&data);
    images::raster_from_data(
        &ImageInfo::new(
            (scan.resolution() as i32, 2),
            ColorType::Gray8,
            AlphaType::Opaque,
            None,
        ),
        data,
        scan.resolution() * 1,
    )
}
