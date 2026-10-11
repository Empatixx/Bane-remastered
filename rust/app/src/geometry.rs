use glam::Vec2;

pub(crate) fn contains(center: Vec2, size: Vec2, point: Vec2) -> bool {
    (point - center).abs().cmple(size / 2.0).all()
}

pub(crate) fn overlaps(a_center: Vec2, a_size: Vec2, b_center: Vec2, b_size: Vec2) -> bool {
    (a_center - b_center)
        .abs()
        .cmple((a_size + b_size) / 2.0)
        .all()
}

pub(crate) fn collides(a_center: Vec2, a_size: Vec2, b_center: Vec2, b_size: Vec2) -> bool {
    (a_center - b_center)
        .abs()
        .cmplt((a_size + b_size) / 2.0)
        .all()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_inside_and_outside() {
        let center = Vec2::new(100.0, 100.0);
        let size = Vec2::new(40.0, 20.0);
        assert!(contains(center, size, Vec2::new(119.0, 105.0)));
        assert!(!contains(center, size, Vec2::new(121.0, 100.0)));
    }

    #[test]
    fn rectangles_overlap_only_when_they_touch() {
        let size = Vec2::new(10.0, 10.0);
        assert!(overlaps(Vec2::ZERO, size, Vec2::new(9.0, 9.0), size));
        assert!(!overlaps(Vec2::ZERO, size, Vec2::new(11.0, 0.0), size));
    }

    #[test]
    fn touching_rectangles_do_not_collide() {
        let size = Vec2::new(10.0, 10.0);
        assert!(collides(Vec2::ZERO, size, Vec2::new(9.0, 0.0), size));
        assert!(!collides(Vec2::ZERO, size, Vec2::new(10.0, 0.0), size));
    }
}
