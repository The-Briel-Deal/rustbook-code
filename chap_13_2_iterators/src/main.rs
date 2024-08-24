fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.into_iter();
    let mut v1_iter = v1_iter;
    v1_iter.next();
    for i in v1_iter {
        println!("Got: {i}")
    }
}
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
