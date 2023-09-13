extern crate num;
use num::Complex;
use std::str::FromStr;

/// Разбирает пару чисел с плавающей точкой, разделенных запятой, и возвращает
/// ее в виде комплексного числа.
pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}
#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// Разбирает строку `s`, содержащую пару координат, например: `"400x600"` или
/// `"1.0,0.5"`.
///
/// Точнее, `s` должна иметь вид <left><sep><right>, где <sep> – символ, заданный
/// в аргументе `separator`, а <left> и <right> – строки, допускающие разбор
/// методом `T::from_str`.
///
/// Если `s` удалось разобрать, то возвращает `Some<(x, y)>`, в противном случае
/// `None`.
pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}
#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}
