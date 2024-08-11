fn main() {
    let nums = [4, 2, 8, 0, 1];
    let target = 9;
    let result: i32 = {
        let mut i1 = 0;
        'found: loop {
            let mut i2 = 0;
            loop {
                if nums[i1] + nums[i2] == target {
                    break 'found target;
                }
                if i2 >= nums.len() - 1 {
                    break;
                }
                i2 += 1;
            }
            if i1 >= nums.len() - 1 {
                break 0;
            }
            i1 += 1;
        }
    };
    println!("Result is {result}")
}
/*
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    let number_two = {
        let mut number = number;
        loop {
            number += 1;
            if number > 8 {
                break number;
            }
        }
    };
    println!("Hello, world! {number} {number_two} ");
}
*/
