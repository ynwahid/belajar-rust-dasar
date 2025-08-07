fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello, test!");
}

#[test]
fn test_variable() {
    let name = "Ucup";
    println!("Hello, {}!", name);
}

#[test]
fn test_mutable_variable() {
    let mut name = "Ucup";
    println!("Hello, {}", name);

    name = "Cup";
    println!("Hello, {}", name);
}

#[test]
fn test_shadowing() {
    let name = "Ucup";
    println!("Hello, {}!", name);

    let name = 10;
    println!("Hello, {}!", name);
}

/*
These are
multiline comments.
*/
#[test]
fn comment() {
    // This is comment
    println!("Hello, comment!");
}

#[test]
fn data_type_explicit() {
    let age: i32 = 23;
    println!("{}", age);

    let price: f32 = 5000.0;
    println!("{}", price);
}

#[test]
fn data_type_number() {
    let a = 10;
    println!("{}", a);

    let b: f32 = 10.5;
    println!("{}", b);
}

#[test]
fn data_type_number_conversion() {
    let a = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    // integer overflow
    let d: i64 = 1_000_000_000;
    let e: i8 = d as i8;
    println!("{}", e);
}
