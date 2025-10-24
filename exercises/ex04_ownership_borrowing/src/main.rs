fn own_and_print_string(msg: String)
{
    println!("'{msg}' is an owned string");
}

fn borrow_and_print_string(msg: &str) 
{
    println!("'{msg}' is a borrowed string");
}

fn main() {
    let owned = String::from("hello");
    own_and_print_string(owned);

    let borrowed = String::from("hello");
    borrow_and_print_string(&borrowed);
}
