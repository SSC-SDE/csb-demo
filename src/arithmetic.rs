/// Clamp out-of-range sums to the nearest representable integer.
pub(crate) fn bounded_sum(left: i32, right: i32) -> i32 {
    left.saturating_add(right)
}

#[cfg(test)]
mod tests {
    use crate::{handle, other};

    #[test]
    fn callers_clamp_at_both_limits() {
        assert_eq!(handle(i32::MAX, 1), i32::MAX);
        assert_eq!(other(i32::MIN, -1), i32::MIN);
        assert_eq!(handle(8, -3), 5);
    }
}
