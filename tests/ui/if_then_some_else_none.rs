#![warn(clippy::if_then_some_else_none)]

fn foo() -> bool {
    unimplemented!()
}

fn bar() -> bool {
    unimplemented!()
}

fn into_some<T>(v: T) -> Option<T> {
    Some(v)
}

fn into_none<T>() -> Option<T> {
    None
}

fn main() {
    let _ = if foo() {
        println!("hi!");
        Some(42)
    } else {
        None
    };

    let _a = linted_1();
    let _ = linted_2();
    let _ = linted_3();

    let _ = not_linted_1();
    let _ = not_linted_2();
    let _ = not_linted_3();
}

fn linted_1() -> Option<Vec<i32>> {
    let mut v = Vec::new();
    if foo() {
        v.push(42);
        Some(v)
    } else {
        None
    }
}

fn linted_2() -> Option<Vec<i32>> {
    let mut v = Vec::new();
    if foo() {
        v.push(42);
        Some(v)
    } else {
        eprintln!("foo is false!");
        None
    }
}

fn linted_3() -> Option<Vec<i32>> {
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

fn not_linted_1() -> Option<i32> {
    if foo() { None } else { Some(42) }
}

fn not_linted_2() -> Option<i32> {
    if foo() { Some(42) } else { into_none() }
}

fn not_linted_3() -> Option<i32> {
    if foo() { into_some(42) } else { None }
}
