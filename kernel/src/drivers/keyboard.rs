use alloc::collections::VecDeque;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref KEYBOARD_QUEUE: Mutex<VecDeque<char>> = Mutex::new(VecDeque::new());
}

pub fn push_char(c: char) {
    let mut queue = KEYBOARD_QUEUE.lock();
    if queue.len() < 100 {
        queue.push_back(c);
    }
}

pub fn pop_char() -> Option<char> {
    KEYBOARD_QUEUE.lock().pop_front()
}
