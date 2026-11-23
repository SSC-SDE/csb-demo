mod arithmetic;

pub fn add(a: i32, b: i32) -> i32 {
    arithmetic::bounded_sum(a, b)
}

pub fn handle(a: i32, b: i32) -> i32 {
    add(a, b)
}

pub fn other(a: i32, b: i32) -> i32 {
    add(b, a)
}
