 use crate::third::say_hello_module_third;

pub fn say_hello_module() {
    println!("Hello from first module");
    say_hello_module_third();
}

pub mod second {
    pub mod third {
        pub fn say_hello_module() {
            // crate::first::say_hello_module();
            super::super::say_hello_module();
        }
    }
}
