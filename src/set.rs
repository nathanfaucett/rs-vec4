use num::Num;
use signed::Signed;


#[inline]
pub fn set<'a, 'b, T: Copy + Num>(out: &'a mut [T; 4], x: T, y: T, z: T, w: T) -> &'a mut [T; 4] {
    out[0] = x;
    out[1] = y;
    out[2] = z;
    out[3] = w;
    out
}
#[test]
fn test_set() {
    let mut v = [0, 0, 0, 0];
    set(&mut v, 1, 2, 3, 4);
    assert!(v == [1, 2, 3, 4]);
}

#[inline]
pub fn zero<'a, 'b, T: Copy + Num>(out: &'a mut [T; 4]) -> &'a mut [T; 4] { set(out, T::zero(), T::zero(), T::zero(), T::zero()) }
#[inline]
pub fn identity<'a, 'b, T: Copy + Num>(out: &'a mut [T; 4]) -> &'a mut [T; 4] { set(out, T::zero(), T::zero(), T::zero(), T::one()) }
#[inline]
pub fn up<'a, 'b, T: Copy + Num>(out: &'a mut [T; 4]) -> &'a mut [T; 4] { set(out, T::zero(), T::zero(), T::one(), T::one()) }
#[inline]
pub fn down<'a, 'b, T: Copy + Signed>(out: &'a mut [T; 4]) -> &'a mut [T; 4] { set(out, T::zero(), T::zero(), -T::one(), T::one()) }
#[inline]
pub fn forward<'a, 'b, T: Copy + Num>(out: &'a mut [T; 4]) -> &'a mut [T; 4] { set(out, T::zero(), T::one(), T::one(), T::one()) }
#[inline]
pub fn back<'a, 'b, T: Copy + Signed>(out: &'a mut [T; 4]) -> &'a mut [T; 4] { set(out, T::zero(), -T::one(), T::zero(), T::one()) }
#[inline]
pub fn right<'a, 'b, T: Copy + Num>(out: &'a mut [T; 4]) -> &'a mut [T; 4] { set(out, T::one(), T::zero(), T::one(), T::one()) }
#[inline]
pub fn left<'a, 'b, T: Copy + Signed>(out: &'a mut [T; 4]) -> &'a mut [T; 4] { set(out, -T::one(), T::zero(), T::one(), T::one()) }
