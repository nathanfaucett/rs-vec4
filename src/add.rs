use num::Num;


#[inline(always)]
pub fn add<T: Num>(out: &mut [T; 4], a: [T; 4], b: [T; 4]) ->  &mut [T; 4] {
    out[0] = a[0] + b[0];
    out[1] = a[1] + b[1];
    out[2] = a[2] + b[2];
    out[3] = a[3] + b[3];
    out
}
#[test]
fn test_add() {
    let mut v = [0, 0, 0, 0];
    add(&mut v, [1, 1, 1, 1], [1, 1, 1, 1]);
    assert!(v[0] == 2);
    assert!(v[1] == 2);
    assert!(v[2] == 2);
    assert!(v[3] == 2);
}

#[inline(always)]
pub fn sadd<T: Num>(out: &mut [T; 4], a: [T; 4], s: T) ->  &mut [T; 4] {
    out[0] = a[0] + s;
    out[1] = a[1] + s;
    out[2] = a[2] + s;
    out[3] = a[3] + s;
    out
}
#[test]
fn test_sadd() {
    let mut v = [0, 0, 0, 0];
    sadd(&mut v, [0, 0, 0, 0], 1);
    assert!(v[0] == 1);
    assert!(v[1] == 1);
    assert!(v[2] == 1);
    assert!(v[3] == 1);
}
