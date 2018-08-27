pub mod closure;
pub mod smart_pointer;
pub mod concurrent;
pub mod my_unsafe;
pub mod server;

pub fn run_learn_closure() {
    closure::run();
}

pub fn run_learn_smart_pointer() {
    smart_pointer::run();
}

pub fn run_learn_concurrent() {
    concurrent::run();
}

pub fn run_learn_unsafe() {
    my_unsafe::run();
}

pub fn run_learn_server() {
    server::run();
}
