/// STRING
/// String allocates dynamically and is resizable

/// So "hello" is not of type String.
/// It is of type &str (pronounced 'string slice').
/// It's like the distinction between const char* and std::string in C++, except &str is much more intelligent.
/// In fact, &str and String have a very similar relationship to each other as do &[T] to Vec<T>.

// string1.rs
fn dump(s: &str) {
    println!("str '{}'", s);
}

fn main() {
    let text = "hello dolly";  // the string slice
    let s = text.to_string();  // it's now an allocated string

    dump(text);
    dump(&s);
}