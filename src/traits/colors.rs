use crate::helpers::round;
use image::Rgba;

pub trait RGBA {
    fn to_float(&self) -> (f32, f32, f32, f32);
    fn to_hsl(&self) -> (f32, f32, f32, f32);
    fn from_hsl(_h: f32, _s: f32, _l: f32, a: f32) -> Self;
    fn from_float(r: f32, g: f32, b: f32, a: f32) -> Self;
    fn normalize_hue(&mut self);
}

impl RGBA for Rgba<u8> {
    fn to_float(&self) -> (f32, f32, f32, f32) {
        let [r, g, b, a] = self.0;
        (
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    fn from_float(r: f32, g: f32, b: f32, a: f32) -> Self {
        if r == g && g == b {
            let shit = round::round_f32(r * 255.0) as u8;
            return Self([shit, shit, shit, (a * 255.0).round() as u8]);
        }
        Self([
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
            (a * 255.0).round() as u8,
        ])
    }

    fn to_hsl(&self) -> (f32, f32, f32, f32) {
        let (r, g, b, alpha) = self.to_float();

        let _max = max!(r, max!(g, b));
        let _min = min!(r, min!(g, b));

        let delta = _max - _min;
        let mut h: f32 = 0.0;
        let s: f32;
        let l: f32 = _min + _max;

        if delta > 0.0 {
            if _max == r {
                h = ((g - b) / delta) % 6.0;
            } else if _max == g {
                h = ((b - r) / delta) + 2.0;
            } else {
                h = ((r - g) / delta) + 4.0;
            }
            h *= 60.0;
            if h < 0.0 {
                h += 360.0;
            }
            s = delta / (1.0 - (l - 1.0).abs());
        } else {
            s = 0.0;
        }

        (h, s * 100.0, l * 50.0, alpha)
    }

    fn from_hsl(_h: f32, _s: f32, _l: f32, a: f32) -> Self {
        let r: f32;
        let g: f32;
        let b: f32;

        let h = if _h >= 360.0 { 6.0 } else { _h / 60.0 };
        let s = if _s > 100.0 { 1.0 } else { _s / 100.0 };
        let l = if _l > 100.0 { 1.0 } else { _l / 100.0 };

        let c = (1.0 - (l * 2.0 - 1.0).abs()) * s;
        let x = c * (1.0 - (h % 2.0 - 1.0).abs());

        let m = l - c / 2.0;

        if (0.0..1.0).contains(&h) {
            r = c + m;
            g = x + m;
            b = m;
        } else if (1.0..2.0).contains(&h) {
            r = x + m;
            g = c + m;
            b = m;
        } else if (2.0..3.0).contains(&h) {
            r = m;
            g = c + m;
            b = x + m;
        } else if (3.0..4.0).contains(&h) {
            r = m;
            g = x + m;
            b = c + m;
        } else if (4.0..5.0).contains(&h) {
            r = x + m;
            g = m;
            b = c + m;
        } else {
            r = c + m;
            g = m;
            b = x + m;
        }

        Self::from_float(r, g, b, a)
    }

    fn normalize_hue(&mut self) {
        let (_h, s, l, a) = self.to_hsl();
        let h = f32::max((_h / 360.0).floor() * -360.0 + _h, 0.0);
        *self = RGBA::from_hsl(h, s, l, a);
    }
}
