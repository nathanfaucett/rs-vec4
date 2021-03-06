use num::Num;
use div::sdiv;


#[inline]
pub fn length_values_sq<'a, 'b, T: Copy + Num>(x: T, y: T, z: T, w: T) -> T {
    x * x + y * y + z * z + w * w
}
#[test]
fn test_length_values_sq() {
    assert!(length_values_sq(1, 1, 1, 1) == 4);
}

#[inline]
pub fn length_values<'a, 'b, T: Copy + Num>(x: T, y: T, z: T, w: T) -> T {
    let lsq = length_values_sq(x, y, z, w);
    if lsq == T::zero() {lsq} else {lsq.sqrt()}
}
#[test]
fn test_length_values() {
    assert!(length_values(1, 1, 1, 1) == 2);
}

#[inline]
pub fn inv_length_values<'a, 'b, T: Copy + Num>(x: T, y: T, z: T, w: T) -> T {
    let lsq = length_values_sq(x, y, z, w);
    if lsq == T::zero() {lsq} else {T::one() / lsq.sqrt()}
}
#[test]
fn test_inv_length_values() {
    assert!(inv_length_values(1.0, 1.0, 1.0, 1.0) == 1.0 / 2.0);
}

#[inline]
pub fn dot<'a, 'b, T: Copy + Num>(a: &'b [T; 4], b: &'b [T; 4]) -> T {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
}
#[test]
fn test_dot() {
    assert!(dot(&[1, 1, 1, 1], &[1, 1, 1, 1]) == 4);
}

#[inline]
pub fn length<'a, 'b, T: Copy + Num>(out: &'b [T; 4]) -> T {
    length_values(out[0], out[1], out[2], out[3])
}
#[test]
fn test_length() {
    assert!(length(&[1, 1, 1, 1]) == 2);
}

#[inline]
pub fn normalize<'a, 'b, T: Copy + Num>(out: &'a mut [T; 4], a: &'b [T; 4]) -> &'a mut [T; 4] {
    sdiv(out, a, length(a))
}
#[test]
fn test_normalize() {
    let mut v = [0.0, 0.0, 0.0, 0.0];
    normalize(&mut v, &[0.0, 0.0, 0.0, 1.0]);
    assert!(v[0] == 0.0);
    assert!(v[1] == 0.0);
    assert!(v[2] == 0.0);
    assert!(v[3] == 1.0);
}
