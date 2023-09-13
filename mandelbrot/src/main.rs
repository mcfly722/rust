extern crate crossbeam;
use std::io::Write;

mod image;
mod parse;
mod render;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        writeln!(
            std::io::stderr(),
            "Порядок вызова: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT"
        )
        .unwrap();
        writeln!(
            std::io::stderr(),
            "Пример: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        )
        .unwrap();
        std::process::exit(1);
    }
    let bounds = parse::parse_pair(&args[2], 'x').expect("ошибка при разборе размеров изображения");
    let upper_left =
        parse::parse_complex(&args[3]).expect("ошибка при разборе координат левого верхнего угла");
    let lower_right =
        parse::parse_complex(&args[4]).expect("ошибка при разборе координат правого нижнего угла");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    //render::render(&mut pixels, bounds, upper_left, lower_right);

    let threads = 1024;
    let rows_per_band = bounds.1 / threads + 1;
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left =
                    render::pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = render::pixel_to_point(
                    bounds,
                    (bounds.0, top + height),
                    upper_left,
                    lower_right,
                );
                spawner.spawn(move || {
                    render::render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        });
    }

    image::write_image(&args[1], &pixels, bounds).expect("ошибка при записи PNG-файла");
}
