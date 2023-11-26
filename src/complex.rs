use std::fmt;
use std::ops::*;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct c64(__m128d);

impl c64 {
    pub fn new(a: f64, b: f64) -> Self {
        Self(unsafe { _mm_set_pd(b, a) })
    }

    pub fn zero() -> Self {
        Self(unsafe { _mm_setzero_pd() })
    }

    pub fn conjugate(&self) -> Self {
        Self(unsafe { _mm_mul_pd(_mm_set_pd(-1.0, 1.0), **self) })
    }

    pub fn abs(&self) -> f64 {
        let m1 = unsafe { _mm_mul_pd(**self, **self) };
        let m2 = unsafe { _mm_permute_pd(m1, 1) };
        let m3 = unsafe { _mm_add_pd(m1, m2) };
        let abs2 = unsafe { _mm_cvtsd_f64(m3) };
        abs2.sqrt()
    }
}

impl fmt::Display for c64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: f64 = unsafe { _mm_cvtsd_f64(**self) };
        let b: f64 = unsafe { _mm_cvtsd_f64(_mm_permute_pd(**self, 1)) };
        write!(f, "{:.3}{:+.3}*i", a, b)
    }
}

impl Deref for c64 {
    type Target = __m128d;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add for c64 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(unsafe { _mm_add_pd(*self, *other) })
    }
}

impl AddAssign for c64 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(unsafe { _mm_add_pd(**self, *other) })
    }
}

impl Div for c64 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let aa = unsafe { _mm_shuffle_pd(*self, *self, 0) };
        let bb = unsafe { _mm_shuffle_pd(*self, *self, 3) };
        let cc = unsafe { _mm_shuffle_pd(*other, *other, 0) };
        let dd = unsafe { _mm_shuffle_pd(*other, *other, 3) };

        let cc2 = unsafe { _mm_mul_pd(cc, cc) };
        let dd2 = unsafe { _mm_mul_pd(dd, dd) };

        let adac = unsafe { _mm_mul_pd(aa, other.0) };
        let bdbc = unsafe { _mm_mul_pd(bb, other.0) };
        let acad = unsafe { _mm_permute_pd(adac, 1) };

        Self(unsafe {
            _mm_div_pd(
                _mm_permute_pd(_mm_addsub_pd(bdbc, acad), 1),
                _mm_add_pd(cc2, dd2),
            )
        })
    }
}

impl DivAssign for c64 {
    fn div_assign(&mut self, other: Self) {
        let aa = unsafe { _mm_shuffle_pd(self.0, self.0, 0) };
        let bb = unsafe { _mm_shuffle_pd(self.0, self.0, 3) };
        let cc = unsafe { _mm_shuffle_pd(other.0, other.0, 0) };
        let dd = unsafe { _mm_shuffle_pd(other.0, other.0, 3) };

        let cc2 = unsafe { _mm_mul_pd(cc, cc) };
        let dd2 = unsafe { _mm_mul_pd(dd, dd) };

        let adac = unsafe { _mm_mul_pd(aa, other.0) };
        let bdbc = unsafe { _mm_mul_pd(bb, other.0) };
        let acad = unsafe { _mm_permute_pd(adac, 1) };

        *self = Self(unsafe {
            _mm_div_pd(
                _mm_permute_pd(_mm_addsub_pd(bdbc, acad), 1),
                _mm_add_pd(cc2, dd2),
            )
        });
    }
}

impl Mul for c64 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let aa = unsafe { _mm_shuffle_pd(*self, *self, 0) };
        let bb = unsafe { _mm_shuffle_pd(*self, *self, 3) };

        let adac = unsafe { _mm_mul_pd(aa, *other) };
        let bdbc = unsafe { _mm_mul_pd(bb, *other) };
        let bcbd = unsafe { _mm_permute_pd(bdbc, 1) };

        Self(unsafe { _mm_addsub_pd(adac, bcbd) })
    }
}

impl MulAssign for c64 {
    fn mul_assign(&mut self, other: Self) {
        let aa = unsafe { _mm_shuffle_pd(**self, **self, 0) };
        let bb = unsafe { _mm_shuffle_pd(**self, **self, 3) };

        let adac = unsafe { _mm_mul_pd(aa, *other) };
        let bdbc = unsafe { _mm_mul_pd(bb, *other) };
        let bcbd = unsafe { _mm_permute_pd(bdbc, 1) };
        *self = Self(unsafe { _mm_addsub_pd(adac, bcbd) });
    }
}

impl Sub for c64 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(unsafe { _mm_sub_pd(*self, *other) })
    }
}

impl SubAssign for c64 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self(unsafe { _mm_sub_pd(**self, *other) })
    }
}
