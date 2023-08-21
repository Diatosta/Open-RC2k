pub mod string;
pub mod file;
pub mod thread;

pub fn inject_hooks() {
    string::inject_hooks();
    file::inject_hooks();
    thread::inject_hooks();
}