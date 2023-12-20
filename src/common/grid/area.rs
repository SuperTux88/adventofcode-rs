use glam::IVec2;
use itertools::Itertools;

/// Calculate the area of a polygon defined by the corners of the perimeter.
/// Using the shoelace formula: https://en.wikipedia.org/wiki/Shoelace_formula
///
/// To also add the outer-half of the perimeter line, you need to add `(premiter / 2) + 1` to get the full area.
pub fn shoelace_area(perimeter: &[IVec2]) -> i64 {
    perimeter
        .iter()
        .circular_tuple_windows()
        .map(|(a, b)| (a.y + b.y) as i64 * (a.x - b.x) as i64)
        .sum::<i64>()
        .abs()
        / 2
}
