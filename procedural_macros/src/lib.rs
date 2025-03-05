use std::sync::{Arc, Condvar, Mutex};

pub struct Sender<T> {}

pub struct Receiver<T> {}

struct Inner<T> {
    queue: Vec<T>,
}
