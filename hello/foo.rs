/// This macro prints "Hello, world!"
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

pub fn foo() {
    say_hello!();
}