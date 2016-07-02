use num::Num;


#[inline(always)]
pub fn new<'a, T: Num>(x: T, y: T, z: T, w: T) -> [T; 4] {[x, y, z, w]}
#[inline(always)]
pub fn create<'a, T: Num>(x: T, y: T, z: T, w: T) -> [T; 4] {new(x, y, z, w)}
#[test]
fn test_new() {
    let v = new(1, 2, 3, 4);
    assert!(v[0] == 1);
    assert!(v[1] == 2);
    assert!(v[2] == 3);
    assert!(v[3] == 4);
}

#[inline(always)]
pub fn clone<'a, T: Num>(v: &'a [T; 4]) -> [T; 4] {new(v[0], v[1], v[2], v[3])}

#[inline(always)]
pub fn copy<'a, T: Num>(out: &'a mut [T; 4], a: &'a [T; 4]) -> &'a mut [T; 4] {
    out[0] = a[0];
    out[1] = a[1];
    out[2] = a[2];
    out[3] = a[3];
    out
}
#[test]
fn test_copy() {
    let mut v = [0, 0, 0, 0];
    copy(&mut v, &[1, 2, 3, 4]);
    assert!(v == [1, 2, 3, 4]);
}
