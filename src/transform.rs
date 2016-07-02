use vec2;
use vec3;
use num::Num;


#[inline(always)]
pub fn transform_mat3<'a, 'b, T: Num>(out: &'a mut [T; 4], a: &'b [T; 4], m: [T; 9]) -> &'a mut [T; 4] {
    out[0] = a[0] * m[0] + a[1] * m[3] + a[2] * m[6];
    out[1] = a[0] * m[1] + a[1] * m[4] + a[2] * m[7];
    out[2] = a[0] * m[2] + a[1] * m[5] + a[2] * m[8];
    out
}

#[inline(always)]
pub fn transform_mat4<'a, 'b, T: Num>(out: &'a mut [T; 4], a: &'b [T; 4], m: [T; 16]) -> &'a mut [T; 4] {
    out[0] = a[0] * m[0] + a[1] * m[4] + a[2] * m[8] + a[3] * m[12];
    out[1] = a[0] * m[1] + a[1] * m[5] + a[2] * m[9] + a[3] * m[13];
    out[2] = a[0] * m[2] + a[1] * m[6] + a[2] * m[10] + a[3] * m[14];
    out[3] = a[0] * m[3] + a[1] * m[7] + a[2] * m[11] + a[3] * m[15];
    out
}

#[inline(always)]
pub fn transform_mat4_rotation<'a, 'b, T: Num>(out: &'a mut [T; 4], a: &'b [T; 4], m: [T; 16]) -> &'a mut [T; 4] {
    out[0] = a[0] * m[0] + a[1] * m[4] + a[2] * m[8];
    out[1] = a[0] * m[1] + a[1] * m[5] + a[2] * m[9];
    out[2] = a[0] * m[2] + a[1] * m[6] + a[2] * m[10];
    out[3] = a[0] * m[3] + a[1] * m[7] + a[2] * m[11] + a[3] * m[15];
    out
}

#[inline(always)]
pub fn transform_projection<'a, 'b, T: Num>(out: &'a mut [T; 4], a: &'b [T; 4], m: [T; 16]) -> &'a mut [T; 4] {
    let mut d = a[0] * m[3] + a[1] * m[7] + a[2] * m[11] + a[3] * m[15];
    d = if d != T::zero() {T::one() / d} else {d};

    out[0] = (a[0] * m[0] + a[1] * m[4] + a[2] * m[8] + a[3] * m[12]) * d;
    out[1] = (a[0] * m[1] + a[1] * m[5] + a[2] * m[9] + a[3] * m[13]) * d;
    out[2] = (a[0] * m[2] + a[1] * m[6] + a[2] * m[10] + a[3] * m[14]) * d;
    out[3] = (a[0] * m[3] + a[1] * m[7] + a[2] * m[11] + a[3] * m[15]) * d;
    out
}

#[inline(always)]
pub fn transform_projection_no_position<'a, 'b, T: Num>(out: &'a mut [T; 4], a: &'b [T; 4], m: [T; 16]) -> &'a mut [T; 4] {
    let mut d = a[0] * m[3] + a[1] * m[7] + a[2] * m[11] + a[3] * m[15];
    d = if d != T::zero() {T::one() / d} else {d};

    out[0] = (a[0] * m[0] + a[1] * m[4] + a[2] * m[8] + a[3]) * d;
    out[1] = (a[0] * m[1] + a[1] * m[5] + a[2] * m[9] + a[3]) * d;
    out[2] = (a[0] * m[2] + a[1] * m[6] + a[2] * m[10] + a[3]) * d;
    out[3] = (a[0] * m[3] + a[1] * m[7] + a[2] * m[11] + a[3] * m[15]) * d;
    out
}

#[inline(always)]
pub fn position_mat32<'a, 'b, T: Num>(out: &'a mut [T; 4], m: [T; 6]) -> &'a mut [T; 4] {
    out[0] = m[4];
    out[1] = m[5];
    out
}

#[inline(always)]
pub fn position_mat4<'a, 'b, T: Num>(out: &'a mut [T; 4], m: [T; 16]) -> &'a mut [T; 4] {
    out[0] = m[12];
    out[1] = m[13];
    out[2] = m[14];
    out[3] = m[15];
    out
}

#[inline(always)]
pub fn scale_mat2<'a, 'b, T: Num>(out: &'a mut [T; 4], m: &'b [T; 4]) -> &'a mut [T; 4] {
    out[0] = vec2::length_values(m[0], m[2]);
    out[1] = vec2::length_values(m[1], m[3]);
    out[2] = T::one();
    out[3] = T::one();
    out
}

#[inline(always)]
pub fn scale_mat32<'a, 'b, T: Num>(out: &'a mut [T; 4], m: [T; 6]) -> &'a mut [T; 4] {
    out[0] = vec3::length_values(m[0], m[2], T::zero());
    out[1] = vec3::length_values(m[1], m[3], T::zero());
    out[2] = T::one();
    out[3] = T::one();
    out
}

#[inline(always)]
pub fn scale_mat3<'a, 'b, T: Num>(out: &'a mut [T; 4], m: [T; 9]) -> &'a mut [T; 4] {
    out[0] = vec3::length_values(m[0], m[3], m[6]);
    out[1] = vec3::length_values(m[1], m[4], m[7]);
    out[2] = vec3::length_values(m[2], m[5], m[8]);
    out[3] = T::one();
    out
}

#[inline(always)]
pub fn scale_mat4<'a, 'b, T: Num>(out: &'a mut [T; 4], m: [T; 16]) -> &'a mut [T; 4] {
    out[0] = vec3::length_values(m[0], m[4], m[8]);
    out[1] = vec3::length_values(m[1], m[5], m[9]);
    out[2] = vec3::length_values(m[2], m[6], m[10]);
    out[3] = T::one();
    out
}
