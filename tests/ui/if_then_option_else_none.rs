#![warn(clippy::if_then_option_else_none)]

fn foo() -> bool {
    unimplemented!()
}

fn bar() -> bool {
    unimplemented!()
}

fn main() {
    let _ = if foo() {
        println!("hi!");
        Some(42)
    } else {
        None
    };

    let _ = function1();

    let _ = function2();

    let _ = function3();
}

fn function1() -> Option<Vec<i32>> {
    let mut v = Vec::new();
    if foo() {
        v.push(42);
        Some(v)
    } else {
        None
    }
}

fn function2() -> Option<Vec<i32>> {
    let mut v = Vec::new();
    if foo() {
        v.push(42);
        Some(v)
    } else {
        eprintln!("foo is false!");
        None
    }
}

fn function3() -> Option<Vec<i32>> {
    let mut v = Vec::new();
    if foo() {
        v.push(0);
        Some(v)
    } else if bar() {
        Some(v)
    } else {
        None
    }
}
