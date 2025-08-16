use core::num;
use std::ops::Not;

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

#[test]
fn for_loop() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("{}", array[index]);
        index += 1
    }

    for value in array {
        println!("{}", value);
    }
}

#[test]
fn range_loop() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    let range = 0..5;
    println!("Start: {}", range.start);
    println!("Start: {}", range.end);

    for i in range {
        println!("{}", array[i]);
    }

    for i in 0..5 {
        println!("{}", array[i]);
    }
}

#[test]
fn range_inclusive_loop() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    let range = 0..=4;
    println!("Start: {}", range.start());
    println!("Start: {}", range.end());

    for i in range {
        println!("{}", array[i]);
    }
}

fn say_hello() {
    println!("Hello!");
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye {} {}!", first_name, last_name);
}

#[test]
fn test_say_hello() {
    say_hello();
    say_goodbye("Ucup", "Nur");
}

fn factorial_loop(n: i32) -> i32 {
    let mut result = 1;
    if n < 1 {
        return result;
    }

    for i in 1..=n {
        result *= i;
    }
    return result;
}

fn factorial_recursive(n: u32) -> u32 {
    if n < 1 {
        return 1;
    }
    return n * factorial_recursive(n - 1);
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(5);
    println!("{}", result);

    let result = factorial_recursive(5);
    println!("{}", result);
}

fn print_number(n: i32) {
    println!("Number: {}", n)
}

fn hi(name: String) {
    println!("Hi {}", name)
}

#[test]
fn function_parameter_ownership() {
    // stack-based values will be cloned, thus we can reuse them
    let number = 10;
    print_number(number);
    println!("{}", number);

    // heap-based values will be transferred, thus we can't reuse them
    let name = String::from("Ucup");
    hi(name);
}

fn full_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn function_return_value_ownership() {
    // stack-based return values will be cloned
    // heap-based return values will be transferred
    let first_name = String::from("Ucup");
    let last_name = String::from("Nur");
    let full_name = full_name(first_name, last_name);
    println!("{}", full_name);
}

fn full_name_tuple(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);
    return (first_name, last_name, full_name);
}

#[test]
fn function_reobtain_parameter_ownership() {
    // heap-based return values will be transferred
    // thus, if we want to reobtain the parameter values, we can use tuple
    let first_name = String::from("Ucup");
    let last_name = String::from("Nur");
    let (first_name, last_name, full_name) = full_name_tuple(first_name, last_name);
    println!("{}", first_name);
    println!("{}", last_name);
    println!("{}", full_name);
}

fn full_name_reference(first_name: &String, last_name: &String) -> String {
    return format!("{} {}", first_name, last_name);
}

#[test]
fn test_full_name_reference() {
    let first_name = String::from("Ucup");
    let last_name = String::from("Nur");

    let full_name = full_name_reference(&first_name, &last_name);
    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

fn change_value(value: &String) {
    // By default we can't modify
    // value.push_str("test");
}

#[test]
fn test_change_value_when_borrowing() {
    let mut value = String::from("Ucup");
    change_value(&value);
    println!("{}", value);
}

fn change_mutable_reference_value(value: &mut String) {
    // By default we can't modify
    value.push_str(" test");
}

#[test]
fn test_change_mutable_reference_value_when_borrowing() {
    let mut value = String::from("Ucup");
    let borrowed_value = &mut value;
    change_mutable_reference_value(borrowed_value);
    change_mutable_reference_value(borrowed_value);
    change_mutable_reference_value(borrowed_value);
    println!("{}", borrowed_value);
}

// dangling reference
// fn get_full_name(first_name: &String, last_name: &String) -> &String {
//     let name = format!("{} {}", first_name, last_name);
//     return &name;
// }

fn get_full_name(first_name: &String, last_name: &String) -> String {
    let name = format!("{} {}", first_name, last_name);
    return name;
}

#[test]
fn test_get_full_name() {
    let first_name = String::from("Ucup");
    let last_name = String::from("Nur");

    let full_name = get_full_name(&first_name, &last_name);
    println!("{}", full_name);
}

#[test]
fn slice_reference() {
    // slice uses reference, thus it doesn't transfer the ownership
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let slice1: &[i32] = &array[..];
    println!("{:?}", slice1);

    let slice2: &[i32] = &array[0..5];
    println!("{:?}", slice2);

    let slice3: &[i32] = &array[5..];
    let slice4 = slice3;
    println!("{:?}", slice3);
}

#[test]
fn string_slice_reference() {
    let name = String::from("Ucup Nur");

    let first_name: &str = &name[0..5];
    println!("{}", first_name);

    let last_name: &str = &name[5..];
    println!("{}", last_name);
}

fn print_person(person: &Person) {
    println!("{}", person.first_name);
    println!("{}", person.mid_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

#[test]
fn struct_person() {
    let person: Person = Person {
        first_name: String::from("Ucup"),
        mid_name: String::from("Nur"),
        last_name: String::from("Wahid"),
        age: 20,
    };
    print_person(&person);
    println!("{:?}", person);

    // init shorthand
    // it will transfer the ownership if the data is stored in heap
    let first_name: String = String::from("Ucup");
    let last_name: String = String::from("Wahid");
    let person: Person = Person {
        first_name,
        mid_name: String::from("Nur"),
        last_name,
        age: 20,
    };
    print_person(&person);
    println!("{:?}", person);

    // init a new instance with another instance's values
    // it will transfer the ownership if the data is stored in heap
    let person2 = Person { ..person };
    print_person(&person2);
    println!("{:?}", person2);

    // init with clone method
    let person3 = Person {
        first_name: person2.first_name.clone(),
        mid_name: person2.mid_name.clone(),
        last_name: person2.last_name.clone(),
        ..person2
    };
    print_person(&person3);
    println!("{:?}", person3);
}

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(-5.001, 10.020);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

struct Nothing;

#[test]
fn test_nothing() {
    let _nothing1: Nothing = Nothing;
    let _nothing2: Nothing = Nothing {};
}

#[derive(Debug)]
struct Person {
    first_name: String,
    mid_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    // prevent transfer ownership by using reference
    fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {}.", name, self.first_name)
    }
}

#[test]
fn test_method() {
    let person = Person {
        first_name: String::from("Ucup"),
        mid_name: String::from("Nur"),
        last_name: String::from("Wahid"),
        age: 20,
    };
    person.say_hello("Riki");
}

struct GeoPoint(f64, f64);

impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        return GeoPoint(long, lat);
    }
}

#[test]
fn test_associated_function() {
    let geo_point: GeoPoint = GeoPoint::new(10.0, 10.0);
    println!("long: {}", geo_point.0);
    println!("lat: {}", geo_point.1);
}

enum Level {
    Regular,
    Premium,
    Platinum,
}

#[test]
fn test_enum() {
    let _level1: Level = Level::Platinum;
    let _level2: Level = Level::Premium;
    let _level3: Level = Level::Regular;
}

enum Payment {
    // debit card number
    DebitCard(String),
    // bank name and account number
    BankTransfer(String, String),
    // e-wallet name and account number
    EWallet(String, String),
}

impl Payment {
    fn Pay(&self, amount: u32) {
        match self {
            Payment::DebitCard(number) => {
                println!("paying with debit card {} amount: {}", number, amount);
            }
            Payment::BankTransfer(name, account_number) => {
                println!(
                    "paying with bank transfer {} {} amount: {}",
                    name, account_number, amount
                );
            }
            Payment::EWallet(name, account_number) => {
                println!(
                    "paying with e-wallet {} {} amount: {}",
                    name, account_number, amount
                );
            }
        }
    }
}

#[test]
fn test_payment() {
    let _payment1: Payment = Payment::DebitCard(String::from("123"));
    _payment1.Pay(10000);

    let _payment2: Payment = Payment::BankTransfer(String::from("BSI"), String::from("123"));
    _payment2.Pay(15000);

    let _payment3: Payment = Payment::EWallet(String::from("Gopay"), String::from("123"));
    _payment3.Pay(12000);
}

#[test]
fn test_enum_match() {
    let level: Level = Level::Regular;
    match level {
        Level::Regular => {
            println!("Regular");
        }
        Level::Premium => {
            println!("Premium");
        }
        Level::Platinum => {
            println!("Platinum");
        }
    }
}

#[test]
fn test_match_value() {
    let name = "Ucup";

    match name {
        "Anemon" => {
            println!("Hello, Anemon");
        }
        "Budi" => {
            println!("Hello, Budi");
        }
        other => {
            println!("Hello, {}", other);
        }
    }

    match name {
        "Anemon" | "Budi" => {
            println!("Hello, Bro");
        }
        other => {
            println!("Hello, {}", other);
        }
    }
}

#[test]
fn test_match_range() {
    let value = 100;
    match value {
        75..=100 => {
            println!("Great");
        }
        50..=74 => {
            println!("Good");
        }
        25..=49 => {
            println!("Not Bad");
        }
        0..=24 => {
            println!("Bad");
        }
        other => {
            println!("Invalid value {}", other);
        }
    }
}

#[test]
fn test_match_struct() {
    let point = GeoPoint::new(0.1, 1.0);

    match point {
        GeoPoint(long, 0.0) => {
            println!("long: {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("latitude: {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("longitude {} latitude {}", long, lat);
        }
    }

    let person: Person = Person {
        first_name: String::from("Ucup"),
        mid_name: String::from("Nur"),
        last_name: String::from("Wahid"),
        age: 20,
    };

    match person {
        Person {
            first_name,
            last_name,
            ..
        } => {
            println!("{} {}", first_name, last_name);
        }
    }
    {}
}

#[test]
fn test_ignore_match() {
    let point = GeoPoint::new(0.1, 1.0);

    match point {
        GeoPoint(long, _) => {
            println!("long: {}", long);
        }
        GeoPoint(_, lat) => {
            println!("latitude: {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("longitude {} latitude {}", long, lat);
        }
    }

    let value = 100;

    match value {
        75..=100 => {
            println!("Great");
        }
        50..=74 => {
            println!("Good");
        }
        25..=49 => {
            println!("Not Bad");
        }
        0..=24 => {
            println!("Bad");
        }
        _ => {
            println!("Invalid value");
        }
    }
}

#[test]
fn test_match_expression() {
    let value = 9;

    let result = match value {
        0 => "nil",
        1 => "one",
        2 => "two",
        _ => "invalid",
    };
    println!("{}", result);
}

type Age = u8;
type IdentityNumber = String;

struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age,
}

#[test]
fn test_customer() {
    let customer = Customer {
        id: String::from("123"),
        name: String::from("Ucup"),
        age: 20,
    };

    println!("{} {} {}", customer.id, customer.name, customer.age);
}
