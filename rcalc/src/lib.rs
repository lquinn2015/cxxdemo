#[cxx::bridge(namespace = rcalc)]
mod ffi {

    struct gCalc {
        ver: u32,
        data: Vec<u32>,
        hm: Vec<String>,
    }

    extern "Rust" {
        type Calc;
        fn grab_calc() -> Box<Calc>;
        fn printVer(a: &gCalc) -> Result<u32>;
        fn mul(&self, a: i32, b: i32) -> i32;
        fn add(a: i32, b: i32) -> i32;
        fn sub(a: i32, b: i32) -> i32;
    }
}

pub struct Calc {}
use crate::ffi::gCalc;
pub fn grab_calc() -> Box<Calc> {
    Box::new(Calc {})
}

impl Calc {
    pub fn mul(&self, a: i32, b: i32) -> i32 {
        a * b
    }
}

pub fn printVer(a: &gCalc) -> Result<u32, std::io::Error> {
    println!("I have a {}", a.ver);
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Welp"))
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}
