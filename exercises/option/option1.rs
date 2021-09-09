// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(std::option::Option::Some(16));
    print_number(std::option::Option::Some(99));

    let mut numbers: [Option<u16>; 5] = [std::option::Option::Some(0); 5];
    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 1235) + 2) / (4 * 16)
        };

        numbers[iter as usize] = std::option::Option::Some(number_to_add);
    }
}
