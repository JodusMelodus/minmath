#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use minmath::complex::complex::Complex;

    #[test]
    fn test_complex_new_and_fields() {
        let z = Complex::new(2.0, 2.0);
        assert_eq!(z.re, 2.0);
        assert_eq!(z.im, 2.0);
    }

    #[test]
    fn test_complex_from_rect() {
        let z = Complex::from_rect(3.0, 4.0);
        assert_eq!(z.re, 3.0);
        assert_eq!(z.im, 4.0);
    }

    #[test]
    fn test_complex_from_polar() {
        let z = Complex::from_polar(3.0, PI / 6.0);
        assert_eq!(z.re, 3.0 * 3.0_f32.sqrt() / 2.0);
        assert_eq!(z.im, 3.0 / 2.0);
    }

    #[test]
    fn test_complex_conj() {
        let z1 = Complex::new(3.0, 4.0);
        let z2 = z1.conj();
        assert_eq!(z2.re, 3.0);
        assert_eq!(z2.im, -4.0);
    }

    #[test]
    fn test_complex_mod() {
        let z = Complex::new(3.0, 4.0);
        let r = z.r#mod();
        assert_eq!(r, 5.0);
    }

    #[test]
    fn test_complex_add() {
        let z1 = Complex::new(3.0, -1.0);
        let z2 = Complex::new(-2.0, 3.0);
        let z3 = z1 + z2;
        assert_eq!(z3.re, 1.0);
        assert_eq!(z3.im, 2.0);
    }

    #[test]
    fn test_complex_sub() {
        let z1 = Complex::new(3.0, -1.0);
        let z2 = Complex::new(-2.0, 3.0);
        let z3 = z1 - z2;
        assert_eq!(z3.re, 5.0);
        assert_eq!(z3.im, -4.0);
    }

    #[test]
    fn test_complex_mul() {
        let z1 = Complex::new(3.0, -1.0);
        let z2 = Complex::new(-2.0, 3.0);
        let z3 = z1 * z2;
        assert_eq!(z3.re, -3.0);
        assert_eq!(z3.im, 11.0);
    }

    #[test]
    fn test_complex_div() {
        let z1 = Complex::new(3.0, -2.0);
        let z2 = Complex::new(-2.0, 3.0);
        let z3 = z1 / z2;
        assert_eq!(z3.re, -12.0 / 13.0);
        assert_eq!(z3.im, -5.0 / 13.0);
    }
}
