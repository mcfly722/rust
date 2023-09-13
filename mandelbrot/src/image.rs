extern crate image;
use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;

/// Записывает буфер `pixels`, размеры которого заданы аргументом `bounds`, в файл
/// с именем `filename`.
pub fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;
    Ok(())
}
