pub mod string;
pub mod thread;
pub mod number;
pub mod datetime;

pub fn inject_hooks() {
    string::inject_hooks();
    thread::inject_hooks();
    number::inject_hooks();
    datetime::inject_hooks();
}