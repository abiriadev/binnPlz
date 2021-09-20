pub struct Calculator;

impl Calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn mul(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn div(a: i32, b: i32) -> i32 {
        a / b
    }

    pub fn rest(a: i32, b: i32) -> i32 {
        a % b
    }

    pub fn square(a: i32) -> i32 {
        a * *2
    }
}
