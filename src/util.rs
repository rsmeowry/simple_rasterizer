use glam::Vec3;

pub trait RoundN {
    fn round_n(&self, n: usize) -> Self;
}

impl RoundN for f32 {
    fn round_n(&self, n: usize) -> Self {
        let p = 10f32.powf(n as f32);
        (self * p).round() / p
    }
}

impl RoundN for f64 {
    fn round_n(&self, n: usize) -> Self {
        let p = 10f64.powf(n as f64);
        (self * p).round() / p
    }
}

#[cfg(test)]
mod tests {
    use crate::util::RoundN;

    #[test]
    fn test_rounding() {
        assert_eq!(0.2335.round_n(0), 0.);
        assert_eq!(0.2335.round_n(1), 0.2);
        assert_eq!(0.2335.round_n(2), 0.23);
        assert_eq!(0.2335.round_n(3), 0.234);
        assert_eq!(0.2335.round_n(4), 0.2335);
    }
}