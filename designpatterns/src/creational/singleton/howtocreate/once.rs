use once_cell::sync::OnceCell;
use std::sync::Mutex;

static ARRAY: OnceCell<Mutex<Vec<i32>>> = OnceCell::new();

fn singleton_init(array: Vec<i32>) {
    ARRAY.get_or_init(|| Mutex::new(array));
}

fn do_a_call() {
    ARRAY.get().unwrap().lock().unwrap().push(1);
}
