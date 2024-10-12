use std::sync::Mutex;

struct ObjectPrototype {
    number: i32,
    string: String,
    array: Vec<i32>,
}

static PRIVATE_STATIC_OBJECT: Mutex<ObjectPrototype> = Mutex::new(ObjectPrototype {
    number: 0,
    string: String::new(),
    array: Vec::new(),
});

pub fn number_set(value: i32) {
    let mut handle = PRIVATE_STATIC_OBJECT.lock().unwrap();
    handle.number = value;
}

pub fn number_get() -> i32 {
    let handle = PRIVATE_STATIC_OBJECT.lock().unwrap();
    handle.number
}

pub fn string_set(string: &str) {
    let mut handle = PRIVATE_STATIC_OBJECT.lock().unwrap();
    handle.string = string.to_owned();
}

pub fn string_get() -> String {
    let handle = PRIVATE_STATIC_OBJECT.lock().unwrap();
    handle.string.to_owned()
}

pub fn array_push(value: i32) {
    let mut handle = PRIVATE_STATIC_OBJECT.lock().unwrap();
    handle.array.push(value);
}

pub fn array_pop() -> i32 {
    let mut handle = PRIVATE_STATIC_OBJECT.lock().unwrap();
    handle.array.pop().unwrap()
}

pub fn array_get(index: usize) -> i32 {
    let handle = PRIVATE_STATIC_OBJECT.lock().unwrap();
    *handle.array.get(index).unwrap()
}

pub fn array_len() -> usize {
    let handle = PRIVATE_STATIC_OBJECT.lock().unwrap();
    handle.array.len()
}
