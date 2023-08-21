pub mod string;
pub mod thread;

pub fn inject_hooks() {
    string::inject_hooks();
    thread::inject_hooks();
}