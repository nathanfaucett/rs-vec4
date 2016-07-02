use num::Num;


#[inline(always)]
pub fn div<'a, 'b, T: Num>(out: &'a mut [T; 4], a: &'b [T; 4], b: &'b [T; 4]) ->  &'a mut [T; 4] {
    out[0] = if b[0] != T::zero() {a[0] / b[0]} else {T::zero()};
    out[1] = if b[1] != T::zero() {a[1] / b[1]} else {T::zero()};
    out[2] = if b[2] != T::zero() {a[2] / b[2]} else {T::zero()};
    out[3] = if b[3] != T::zero() {a[3] / b[3]} else {T::zero()};
    out
}
#[test]
fn test_div() {
    let mut v = [0, 0, 0, 0];
    div(&mut v, &[1, 1, 1, 1], &[1, 1, 1, 1]);
    assert!(v[0] == 1);
    assert!(v[1] == 1);
    assert!(v[2] == 1);
    assert!(v[3] == 1);
}

#[inline(always)]
pub fn sdiv<'a, 'b, T: Num>(out: &'a mut [T; 4], a: &'b [T; 4], s: T) ->  &'a mut [T; 4] {
    let not_zero = s != T::zero();
    out[0] = if not_zero {a[0] / s} else  {T::zero()};
    out[1] = if not_zero {a[1] / s} else  {T::zero()};
    out[2] = if not_zero {a[2] / s} else  {T::zero()};
    out[3] = if not_zero {a[3] / s} else  {T::zero()};
    out
}
#[test]
fn test_sdiv() {
    let mut v = [0, 0, 0, 0];
    sdiv(&mut v, &[1, 1, 1, 1], 1);
    assert!(v[0] == 1);
    assert!(v[1] == 1);
    assert!(v[2] == 1);
    assert!(v[3] == 1);
}
