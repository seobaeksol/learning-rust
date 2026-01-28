use std::any::type_name_of_val;

fn main() {
    println!("Test FnOnce...");
    test_fn_once();
    println!("");

    println!("Test FnMut 1...");
    test_fn_mut();
    println!("");

    println!("Test FnMut 2...");
    test_fn_mut2();
    println!("");

    println!("Test Fn...");
    test_fn();
    println!("");
}

fn assert_fn<F: Fn()>(f: &F) {
    println!("{} implements Fn", type_name_of_val(f));
}
fn assert_fn_mut<F: FnMut()>(f: &F) {
    println!("{} implements FnMut", type_name_of_val(f));
}
fn assert_fn_once<F: FnOnce()>(f: &F) {
    println!("{} implements FnOnce", type_name_of_val(f));
}

/// `FnOnce` applies toi clousres that can be called once.
/// All clousres implement at least this trait because all closures can be called.
/// A closure that moves caputred values out of its body will only implement `FnOnce` and none of the other `Fn` traits because it can only be called once.
fn test_fn_once() {
    let data = vec!["hello", "world"];

    let scan_data = || {
        for item in data {
            println!("{}", item);
        }
    };

    // assert_fn(&scan_data); // error
    // assert_fn_mut(&scan_data); // error
    assert_fn_once(&scan_data);

    scan_data();
    // scan_data(); // It should occure an ownership error.
}

/// `FnMut` applies to closures that don't move captured values out of their body but might mutate the captured values.
/// These closures can be called more than once
fn test_fn_mut() {
    let mut data = vec!["hello", "world"];

    let mut scan_data = || {
        for item in &data {
            println!("{}", item);
        }

        data.push("Nice to meet you.");
    };

    // assert_fn(&scan_data); // error
    assert_fn_mut(&scan_data);
    assert_fn_once(&scan_data);

    scan_data();
    scan_data();
}

fn test_fn_mut2() {
    let data = "Hello world!".to_string();
    let mut collection = Vec::new();

    let mut push = || {
        collection.push(data.clone());
    };

    // assert_fn(&push); // error
    assert_fn_mut(&push);
    assert_fn_once(&push);

    push();

    println!("{:?}", collection);
}

/// `Fn` applies to closures that don't move captured values out of their body and don't mutate captured values, as well as closures that capture nothing from their environment.
/// These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.
fn test_fn() {
    let data = vec!["hello", "world"];

    let scan_data = || {
        for item in &data {
            println!("{}", item);
        }
    };

    assert_fn(&scan_data);
    assert_fn_mut(&scan_data);
    assert_fn_once(&scan_data);

    scan_data();
    scan_data();
}
