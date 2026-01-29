use std::thread;

fn main() {
    let data = [10, 1, 30];

    // we need `move` keyword to avoid compiler error beacuse the closure leaves current scope. So the compiler doesn't guarantee that `data` live when the closure runs
    // Above sentance is my guess. I am trying to understand how closures work... If there's misunderstand, tell me please.
    let handle = thread::spawn(move || {
        for item in data.into_iter() {
            println!("{}", item);
        }
    });

    handle.join().unwrap();
}
