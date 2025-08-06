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
fn test_shadowing(){
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