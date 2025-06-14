use crate::float2::Float2;

pub fn point_on_right_side_of_line(a: Float2, b: Float2, p: Float2) -> bool {
    let ap = p - a;
    let ab = b - a;
    let ab_rotated = ab.rotate_clockwise();
    ap.dot(&ab_rotated) >= 0.0
}
