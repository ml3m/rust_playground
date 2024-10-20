use std::u8;

fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    let phone_number = 123;
    call_me(phone_number);
}
