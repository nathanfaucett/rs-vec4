use num::Num;


#[inline(always)]
pub fn add<'a, T: Num>(out: &'a mut [T; 4], a: &'a [T; 4], b: &'a [T; 4]) ->  &'a mut [T; 4] {
    out[0] = a[0] + b[0];
    out[1] = a[1] + b[1];
    out[2] = a[2] + b[2];
    out[3] = a[3] + b[3];
    out
}
#[test]
fn test_add() {
    let mut v = [0, 0, 0, 0];
    add(&mut v, &[1, 1, 1, 1], &[1, 1, 1, 1]);
    assert!(v[0] == 2);
    assert!(v[1] == 2);
    assert!(v[2] == 2);
    assert!(v[3] == 2);
}

#[inline(always)]
pub fn sadd<'a, T: Num>(out: &'a mut [T; 4], a: &'a [T; 4], s: T) ->  &'a mut [T; 4] {
    out[0] = a[0] + s;
    out[1] = a[1] + s;
    out[2] = a[2] + s;
    out[3] = a[3] + s;
    out
}
#[test]
fn test_sadd() {
    let mut v = [0, 0, 0, 0];
    sadd(&mut v, &[0, 0, 0, 0], 1);
    assert!(v[0] == 1);
    assert!(v[1] == 1);
    assert!(v[2] == 1);
    assert!(v[3] == 1);
}
