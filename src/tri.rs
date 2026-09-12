use glam::Vec2;

pub struct Tri(pub Vec2, pub Vec2, pub Vec2);

impl Tri {
    pub fn area(&self) -> f32 {
        0.5 *
            (
                self.0.x * (self.1.y - self.2.y)
                    + self.1.x * (self.2.y - self.0.y)
                    + self.2.x * (self.0.y - self.1.y)
            ).abs()
    }

    pub fn barycentric_weights(&self, p: Vec2) -> (f32, f32, f32) {
        let pbc = Tri(p, self.1, self.2);
        let pca = Tri(p, self.2, self.0);
        let pab = Tri(p, self.0, self.1);
        let area = self.area();

        (pbc.area() / area, pca.area() / area, pab.area() / area)
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;
    use crate::tri::Tri;
    use crate::util::RoundN;

    #[test]
    fn test_barycentric_weights() {
        let test_data = [
            (Tri(Vec2::new(1., 2.), Vec2::new(5., 2.), Vec2::new(1., 5.)), Vec2::new(2.2, 3.5), (0.2, 0.3, 0.5)),
            (Tri(Vec2::new(0., 0.), Vec2::new(4., 0.), Vec2::new(0., 3.)), Vec2::new(1., 1.), (0.417, 0.25, 0.333)),
            (Tri(Vec2::new(-1., 0.), Vec2::new(3., 0.), Vec2::new(0., 4.)), Vec2::new(1., 0.), (0.5, 0.5, 0.)),
            (Tri(Vec2::new(-1., 0.), Vec2::new(3., 0.), Vec2::new(0., 4.)), Vec2::new(0., 4.), (0., 0., 1.)),
        ];

        for (tri, point, weights) in test_data {
            let result = tri.barycentric_weights(point);
            assert_eq!(result.0.round_n(3), weights.0);
            assert_eq!(result.1.round_n(3), weights.1);
            assert_eq!(result.2.round_n(3), weights.2);
        }
    }
}