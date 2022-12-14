use std::thread;

const NUMBER_OF_THREADS: u32 = 10;

fn main() {
    // Make a vector to hold the children which are spawn
    let mut children = vec![];

    for i in 0..NUMBER_OF_THREADS {
        children.push(thread::spawn(move || {
            println!("This is thread number: {}", i);
            i
        }));
    }

    for child in children {
        // Wait for the thread to finish, returns a result
        let result = child.join();
        match result {
            Ok(3) => println!("This is number 3"),
            _ => println!("This is not 3"),
        }
    }
}
