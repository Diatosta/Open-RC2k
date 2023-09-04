pub mod datetime;
pub mod number;
pub mod random;
pub mod string;
pub mod thread;

pub fn inject_hooks() {
    string::inject_hooks();
    thread::inject_hooks();
    number::inject_hooks();
    datetime::inject_hooks();
    random::inject_hooks();
}
