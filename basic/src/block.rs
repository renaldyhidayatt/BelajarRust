use rand::Rng;

pub fn blockrust() {
    let a: i32 = {
        let n = rand::thread_rng().gen_range(0..100);
        n * 2
    };

    println!("a: {}", a);
}
