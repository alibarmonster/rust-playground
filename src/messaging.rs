pub use self::message::say_hello as sayHello;

mod message {
    pub fn say_hello(name: &String) {
        println!("Hello {}", name);
    }
}
