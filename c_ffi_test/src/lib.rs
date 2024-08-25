// pub extern "C" fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

#[no_mangle]
pub extern "C" fn hello() {
    println!("Hi from rust");
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
