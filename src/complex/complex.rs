#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    pub fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    pub fn from_rect(x: f32, y: f32) -> Self {
        Self { re: x, im: y }
    }

    pub fn from_polar(r: f32, theta: f32) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    pub fn conj(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn r#mod(&self) -> f32 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
}
