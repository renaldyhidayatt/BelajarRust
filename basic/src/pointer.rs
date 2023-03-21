use rand::Rng;

pub fn pointer() {
    let mut number: i32 = 24;
    println!("value: {:?}", number);

    for _ in 0..=5 {
        change_value(&mut number);
        println!("number: {}", number);
    }

    // let pointer_number: &i32 = &number;
    // println!("pointer: {:p}", pointer_number);

    // let underlying_value = *pointer_number;
    // println!("value : {:}", underlying_value);

    // let mut number = 24;
    // println!("number: {}", number);

    // let pointer_number = &mut number;
    // println!("pointer_number: {:p}", pointer_number);

    // *pointer_number = 12;

    // println!("*pointer_numbher: {}", pointer_number);
    // println!("number: {}", number);

    // let number_one = 24;
    // let number_two = &number_one;
}

fn change_value(n: &mut i32) {
    *n = generate_random_number();
}

fn generate_random_number() -> i32 {
    return rand::thread_rng().gen_range(0..100);
}
