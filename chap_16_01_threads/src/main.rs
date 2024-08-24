use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();
    {
        let tx = tx.clone();
        let _handle = std::thread::spawn(move || {
            let words = vec![
                String::from("Dog"),
                String::from("Cat"),
                String::from("Boogie"),
            ];
            for word in words {
                tx.send(word).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
    }
    {
        let tx = tx.clone();
        let _handle = std::thread::spawn(move || {
            let words = vec![
                String::from("WAHHHHOOOO"),
                String::from("GIIIII"),
                String::from("EDEEEEE"),
            ];
            for word in words {
                tx.send(word).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
    }
    drop(tx);
    for recieved in rx {
        println!("What the dog doin? {recieved}");
    }
}

/* Old Goofy Code
 * for i in 1..1000000 {
 *     thread::spawn(move || {
 *         for j in 1..100000 {
 *             println!("printing {i} {j}");
 *         }
 *         sleep(Duration::from_secs(1));
 *         println!("printing {i} ROUND 2");
 *     });
 * }
 * sleep(Duration::from_secs(10));
 */
