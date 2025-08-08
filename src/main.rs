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

#[test]
fn numeric_operator() {
    let a = 10;
    let b = 10;
    let c = a * b;
    let d = a / b;
    let e = a + b;
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
}

#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn data_type_boolean() {
    let a = true;
    let b: bool = false;
    println!("{}", a);
    println!("{}", b);
}

#[test]
fn comparison_operator() {
    let a = 10;
    let b = 20;
    let result = a > b;
    println!("{}", result);
}

#[test]
fn logical_operator() {
    let absen = 70;
    let nilai_akhir = 70;

    let lulus_absen: bool = absen >= 75;
    let lulus_nilai_akhir: bool = nilai_akhir >= 75;

    let lulus = lulus_absen && lulus_nilai_akhir;
    println!("{}", lulus);
}

#[test]
fn char_type() {
    let char1: char = 'a';
    let char2: char = 'b';
    println!("{} {}", char1, char2);
}

#[test]
fn tuple_data_type() {
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    // accessing tuple elements
    let a = data.0;
    let b = data.1;
    let c = data.2;
    println!("{} {} {}", a, b, c);

    // destructuring tuple
    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);

    // use underscore if the element is unused
    let (a, b, _) = data;
    println!("{} {}", a, b);

    // mutable tuple
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    data.0 = 20;
    data.1 = 11.5;
    data.2 = false;

    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);
}

fn unit() {
    println!("Hello, world!");
}

#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);

    let test: () = unit();
    println!("{:?}", test);
}

#[test]
fn array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    // mutable array
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    array[0] = 10;
    array[0] = 20;
    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    // array length
    let length: usize = array.len();
    println!("{}", length);
}

#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix[0][2]);
}

const MAXIMUM: i32 = 100;
#[test]
fn constant() {
    const MINIMUM: i32 = 0;
    println!("{} {}", MINIMUM, MAXIMUM);
}

#[test]
fn variable_scope() {
    let ucup = 1;

    {
        // inner scope
        println!("inner ucup: {}", ucup);
        let nur = 2;
        println!("inner nur: {}", nur);
    }
    // outer scope can't access inner scope
    // println!("{}", nur)
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Ucup");
    println!("{}, {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("Nur");
    println!("{}, {}", a, b);
}

#[test]
fn string_slice() {
    let name: &str = " Ucup Nur ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);

    let mut username: &str = "Ucup";
    username = "Budi";
    println!("{}", username);
}

#[test]
fn string_type() {
    let mut name: String = String::from("Ucup");
    println!("{}", name);

    name.push_str(" Nur");
    println!("{}", name);

    let budi = name.replace("Ucup", "Budi");
    println!("{}", name);
    println!("{}", budi);
}

#[test]
fn data_copy() {
    let a = 10;

    // ownership does not transferred
    // because it's stack-based variable, thus uses copy method
    let mut b = a;
    println!("{} {}", a, b);
    b = 12;
    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1 = String::from("Ucup");
    println!("{}", name1);

    // ownership transferred from name1 to name2
    let name2 = name1;
    println!("{}", name2);
}

#[test]
fn clone() {
    let name1 = String::from("Ucup");
    println!("{}", name1);

    // ownership does not transferred from name1 to name2
    // because it uses clone method
    let name2 = name1.clone();
    println!("{}", name1);
    println!("{}", name2);
}

#[test]
fn if_expression() {
    let value = 9;
    let result: &str;

    if value >= 9 {
        result = "Good";
    } else if value >= 6 {
        result = "Not Bad";
    } else if value >= 3 {
        result = "Bad";
    } else {
        result = "Very Bad";
    }
    println!("{}", result)
}

#[test]
fn let_if_expression() {
    let value = 9;
    let result: &str = if value >= 9 {
        "Good"
    } else if value >= 6 {
        "Not Bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "Very Bad"
    };
    println!("{}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;

        if counter > 10 {
            break;
        }

        if counter % 2 == 0 {
            continue;
        }
        println!("{}", counter);
    }
}

#[test]
fn let_loop_expression() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };
    println!("{}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }
            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("{}", counter);
        }
        counter += 1;
    }
}
