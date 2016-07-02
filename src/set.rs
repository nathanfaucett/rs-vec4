use num::Num;


#[inline(always)]
pub fn set<'a, T: Num>(out: &'a mut [T; 4], x: T, y: T, z: T, w: T) -> &'a mut [T; 4] {
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

#[inline(always)]
pub fn zero<'a, T: Num>(out: &'a mut [T; 4]) -> &'a mut [T; 4] { set(out, T::zero(), T::zero(), T::zero(), T::zero()) }
#[inline(always)]
pub fn identity<'a, T: Num>(out: &'a mut [T; 4]) -> &'a mut [T; 4] { set(out, T::zero(), T::zero(), T::zero(), T::one()) }
#[inline(always)]
pub fn up<'a, T: Num>(out: &'a mut [T; 4]) -> &'a mut [T; 4] { set(out, T::zero(), T::zero(), T::one(), T::one()) }
#[inline(always)]
pub fn down<'a, T: Num>(out: &'a mut [T; 4]) -> &'a mut [T; 4] { set(out, T::zero(), T::zero(), -T::one(), T::one()) }
#[inline(always)]
pub fn forward<'a, T: Num>(out: &'a mut [T; 4]) -> &'a mut [T; 4] { set(out, T::zero(), T::one(), T::one(), T::one()) }
#[inline(always)]
pub fn back<'a, T: Num>(out: &'a mut [T; 4]) -> &'a mut [T; 4] { set(out, T::zero(), -T::one(), T::zero(), T::one()) }
#[inline(always)]
pub fn left<'a, T: Num>(out: &'a mut [T; 4]) -> &'a mut [T; 4] { set(out, -T::one(), T::zero(), T::one(), T::one()) }
#[inline(always)]
pub fn right<'a, T: Num>(out: &'a mut [T; 4]) -> &'a mut [T; 4] { set(out, T::one(), T::zero(), T::one(), T::one()) }
