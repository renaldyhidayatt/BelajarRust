pub fn arrayRust() {
    let mut numbers = [24, 12, 32, 7];
    println!("array {:?}", numbers);

    let data0 = numbers[0];
    println!("element array ke 0 {data0}");

    let data1 = numbers[1];
    println!("element array ke 1 {data1}");

    numbers[1] = 16;
    numbers[3] = 8;

    println!("array {numbers:?}");

    // let mut alphabets = ["a", "b", "c", "d"];

    // let booleans = [true, false];

    // let floatingNumbers = [32.3, 3.14, 0.5];

    let angka_integer = [24, 12, 32, 7];
    println!("{angka_integer:?}");

    let angka_float = [24.2, 12.5, 32.00002, 7.2];
    println!("{angka_float:?}");

    let data_boolean: [bool; 2] = [false, true];

    println!("{data_boolean:?}");

    let angka_unsigned_integer: [u32; 3] = [24, 0, 12];

    println!("{angka_unsigned_integer:?}");

    let names = ["jason", "grayon", "drake", "damian"];

    let length = names.len();

    println!("array size is {}", length);

    let names: [&str; 4] = ["jason", "grayon", "drake", "damian"];

    for i in 0..names.len() {
        println!("array index ke-{}: {}", i, names[i]);
    }

    let mut i = 0;
    while i < names.len() {
        println!("array index ke-{}: {}", i, names[i]);
    }

    loop {
        if i >= names.len() {
            break;
        }
        println!("array index ke-{}: {}", i, names[i]);

        i += 1;
    }

    for (i, name) in names.iter().enumerate() {
        println!("array index ke-{i}: {name}");
    }

    let data_arr = [
        ["salad", "fried rice"],
        ["apple", "coconut"],
        ["spinach", "jalapeno"],
    ];

    for sub_arr in data_arr {
        for el in sub_arr {
            print!("{el}, ");
        }
        println!();
    }
}
