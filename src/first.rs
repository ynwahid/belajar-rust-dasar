use crate::third::say_hello_module_third;

pub fn say_hello_module() {
    println!("Hello from first module");
    say_hello_module_third();
}
