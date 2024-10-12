mod static_object;

fn main() {
    print_static_object();
    static_object::number_set(12);
    print_static_object();
    static_object::string_set("Hello");
    print_static_object();
    static_object::array_push(123);
    static_object::array_push(456);
    static_object::array_push(789);
    static_object::array_pop();
    print_static_object();
}

fn print_static_object() {
    println!("static_object:");
    println!("  Number = {}", static_object::number_get());
    println!("  String = \"{}\"", static_object::string_get());
    let length = static_object::array_len();
    println!("  Array length = {}", length);
    let mut index: usize = 0;
    loop {
        if index >= length {
            break;
        }
        println!("    Array[{}] = {}", index, static_object::array_get(index));
        index += 1;
    }
    println!("");
}
