extern crate libm;

const F32_EXP_BIAS: u32 = 0x7f;
const F32_EXP_MASK: u32 = 0xff << 23;
const F32_SIGN_MASK: u32 = 1 << 31;
const F32_MANTISSA_BITS: u32 = 23;

pub fn copysign_f32(x: f32, y: f32) -> f32 {
    let iy = y.to_bits();
    let ix = (x.to_bits() & !(1 << 31)) | (iy & (1 << 31));
    f32::from_bits(ix)
}

#[inline(always)]
pub fn libm_roundf(x: f32) -> f32 {
    libm::roundf(x)
}
#[inline(always)]
pub fn libm_trunc(x: f32) -> f32 {
    libm::truncf(x)
}
pub fn trunc_f32(x: f32) -> f32 {
    let mut ix = x.to_bits();
    let exp = (ix & F32_EXP_MASK) >> F32_MANTISSA_BITS;
    if exp >= F32_EXP_BIAS + F32_MANTISSA_BITS {
        return x;
    }
    if exp < F32_EXP_BIAS {
        return x * 0.0;
    }
    let shift = F32_EXP_BIAS + F32_MANTISSA_BITS - exp;
    ix &= (!0) << shift;
    f32::from_bits(ix)
}

pub fn round_f32(x: f32) -> f32 {
    const ROUND_STEP_F32: f32 = 0.5 - 0.25 * std::f32::EPSILON;
    if x.is_nan() {
        return x;
    }
    trunc_f32(x + ROUND_STEP_F32)
}

pub fn round_f32_unwrapped(x: f32) -> f32 {
    const F32_ROUND_STEP: f32 = 0.5 - 0.25 * std::f32::EPSILON;
    let mut ix = x.to_bits();
    let mut exp = (ix & F32_EXP_MASK) >> F32_MANTISSA_BITS;
    if exp >= F32_EXP_BIAS + F32_MANTISSA_BITS {
        return x;
    }
    let step = f32::from_bits(F32_ROUND_STEP.to_bits() | (ix & F32_SIGN_MASK));
    ix = (x + step).to_bits();
    exp = (ix & F32_EXP_MASK) >> F32_MANTISSA_BITS;
    if exp < F32_EXP_BIAS {
        return x * 0.0;
    }
    let shift = F32_EXP_BIAS + F32_MANTISSA_BITS - exp;
    ix &= (!0) << shift;
    f32::from_bits(ix)
}

pub fn round_f32_std_trunc(x: f32) -> f32 {
    const ROUND_STEP_F32: f32 = 0.5 - 0.25 * std::f32::EPSILON;
    if x.is_nan() {
        return x;
    }
    (x + copysign_f32(ROUND_STEP_F32, x)).trunc()
}
