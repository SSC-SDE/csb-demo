pub fn add(a: i32, b: i32) -> i32 {
    a.checked_add(b).unwrap_or(0)
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn handle(a: i32, b: i32) -> i32 {
    add(a, b)
}

pub fn other(a: i32, b: i32) -> i32 {
    add(b, a)
}
