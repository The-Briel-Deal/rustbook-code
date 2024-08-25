extern "C" {
    fn abs(input: i8) -> i8;
}

fn main() {
    unsafe { println!("Absolute value of -3 according to C: {}", abs(-3)) }
}

/*
 * let mut vector = vec![1, 2, 3, 4, 5, 6];
 * let (left, right) = split_at_mut(&mut vector, 3);
 * println!("left: {left:#?}, right: {right:#?}");
 */
/*
 * fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
 *     let len = values.len();
 *     let ptr = values.as_mut_ptr();
 *
 *     assert!(mid <= len);
 *
 *     unsafe {
 *         (
 *             slice::from_raw_parts_mut(ptr, mid),
 *             slice::from_raw_parts_mut(ptr.add(mid), len - mid),
 *         )
 *     }
 * }
 */
