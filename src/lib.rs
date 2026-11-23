mod batch;
pub use batch::process_batch;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn handle(a: i32, b: i32) -> i32 {
    add(a, b)
}

pub fn other(a: i32, b: i32) -> i32 {
    add(b, a)
}
