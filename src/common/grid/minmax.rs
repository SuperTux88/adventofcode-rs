use glam::IVec2;

pub fn minmax_ivec2<'a>(vecs: impl Iterator<Item = &'a IVec2>) -> (IVec2, IVec2) {
    let mut min = IVec2::new(i32::MAX, i32::MAX);
    let mut max = IVec2::new(i32::MIN, i32::MIN);

    for vec in vecs {
        min = min.min(*vec);
        max = max.max(*vec);
    }

    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minmax_ivec2() {
        let vecs = vec![
            IVec2::new(-5, 0),
            IVec2::new(0, 0),
            IVec2::new(3, 3),
            IVec2::new(-2, 2),
            IVec2::new(2, -3),
            IVec2::new(5, 2),
        ];

        let (min, max) = minmax_ivec2(vecs.iter());

        assert_eq!(min, IVec2::new(-5, -3));
        assert_eq!(max, IVec2::new(5, 3));
    }
}
