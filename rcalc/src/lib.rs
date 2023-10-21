#[cxx::bridge(namespace = rcalc)]
mod ffi {

    extern "Rust" {
        type Calc;
        fn grab_calc() -> Box<Calc>;
        fn mul(&self, a: i32, b: i32) -> i32;
        fn add(a: i32, b: i32) -> i32;
        fn sub(a: i32, b: i32) -> i32;
    }
}

pub struct Calc {}

pub fn grab_calc() -> Box<Calc> {
    Box::new(Calc {})
}

impl Calc {
    pub fn mul(&self, a: i32, b: i32) -> i32 {
        a * b
    }
}
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}
