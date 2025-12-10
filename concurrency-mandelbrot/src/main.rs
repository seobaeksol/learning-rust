use num::Complex;

/// It determins whether the `c` belongs to mandelbrot set and the loop count for calculating is limited by `limit`
///
/// `Some(i)` is returned when the `c` doesn't belong to mandelbrot. Here, `i` is the number of iterations it took for `c` to escape the circle of radius 2 centered at the origin.
/// If `c` appears to belong to the Mandelbrot set (more precisely, if we cannnot prove by the time the iteration count reaches the limit that `c` is not in th Mandelbrot set), it returns `None`
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

use std::str::FromStr;

/// It parses `s` as a pair of coordinate like `"400x400"` or `"1.0,0.5"`
///
/// `s` should be form like <left><step><right>. <sep> is a string come from `separator` parameter and <left> and <right> are strings parsed by `T::from_str`
///
/// If `s` is a correct format, it returns `Some<(x, y)>`. If not, returns `None`
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
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

/// It parses a pair of comma-separated floating-point numbers into a complex number
fn parse_complex(s: &str) -> Option<Complex<f64>> {
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

fn main() {
    println!("Hello, world!");
}
