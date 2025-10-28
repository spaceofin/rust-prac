use std::fmt;

#[cfg(target_family="windows")]
#[link(name = "msvcrt")]
unsafe extern "C" {
    // this is a foreign function
    // that computes the square root of a single complex number
    fn csqrtf(z: Complex) -> Complex;
    fn ccosf(z: Complex) -> Complex;
    fn sqrtf(x: f32) -> f32;
}

#[cfg(target_family = "unix")]
#[link(name = "m")]
unsafe extern "C" {
    // this is a foreign function
    // that computes the square root of a single precision complex number
    fn csqrtf(z: Complex) -> Complex;
    fn ccosf(z: Complex) -> Complex;
}

// Since calling foreign functions is considered unsafe,
// it's common to write safe wrappers around them.
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

// Minimal implementation of single precision complex numbers
#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}

pub fn ffi_demo() {
    // z = -1 + 0i
    let z = Complex { re: -1., im: 0. };
    // calling a foreign function is an unsafe operation
    let z_sqrt = unsafe { csqrtf(z) };

    println!("the square root of {:?} is {:?}", z, z_sqrt);

    // calling safe API wrapped around unsafe opration
    println!("cos({:?}) = {:?}", z, cos(z));

    let num = 16.0;
    let sqrt_result = unsafe { sqrtf(num) };
    println!("square root of {} is {}", num, sqrt_result);
}