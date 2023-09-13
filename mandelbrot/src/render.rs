extern crate num;
use num::Complex;

/// Рисует прямоугольную часть множества Мандельброта в буфере пикселей.
///
/// Аргумент `bounds` задает ширину и высоту буфера `pixels`, в котором каждый байт
/// представляет один полутоновый пиксель. Аргументы `upper_left` и `lower_right`
/// определяют точки на комплексной плоскости, соответствующие левому верхнему
/// и правому нижнему углам буфера пикселей.
pub fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

/// Зная строку и столбец пикселя выходного изображения, возвращает соответствующую
/// точку на комплексной плоскости.
///
/// `bounds` - пара, определяющая ширину и высоту изображения в пикселях.
/// `pixel` - пара (строка, столбец), определяющая конкретный пиксель изображения.
/// Параметры `upper_left` и `lower_right` - точки на комплексной плоскости,
/// описывающие область, покрываемую изображением.
pub fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64, // Почему здесь вычитание? pixel.1 увеличивается при движении вниз,
                                                                       // тогда как мнимая часть увеличивается при движении вверх.
    }
}
#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );
}

/// Пытается определить, принадлежит ли `c` множеству Мандельброта, ограничившись
/// `limit` итерациями.
///
/// Если `c` не принадлежит множеству, вернуть `Some(i)`, где `i` – число итераций,
/// понадобившееся для того, чтобы `c` покинула круг радиуса 2 с центром в начале
/// координат. Если `c` может принадлежать множеству (точнее, если после limit итераций
/// не удалось доказать, что `c` не является элементом множества), то вернуть `None`.
pub fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}
