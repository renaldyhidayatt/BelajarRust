use std::collections::VecDeque;

pub fn vectorRust() {
    let mut data_one = vec!["batman", "superman", "lobo"];

    data_one.pop();
    data_one.remove(1);
    data_one.push("rust");
    data_one.push("rust");

    data_one[2] = "main dota";

    let is_vector_empty = data_one.is_empty();

    println!("data: {:?}", data_one);
    println!(
        "length: {}, capacity: {}",
        data_one.len(),
        data_one.capacity()
    );
    println!("result: {:?}", is_vector_empty);
    data_one.clear();
    println!("data: {:?}", data_one);
    println!(
        "length: {}, capacity: {}",
        data_one.len(),
        data_one.capacity()
    );

    let mut result_one = vec![3, 1, 3];
    let mut data_two = vec![3, 1, 3];

    result_one.append(&mut data_two);
    result_one.append(&mut vec![4, 5]);

    println!("data: {:?}", result_one);
    println!(
        "length: {}, capacity: {}",
        result_one.len(),
        result_one.capacity()
    );

    result_one.sort();
    println!("data: {:?}", result_one);

    // let mut vector_4 = vec![1, 2, 3];
    // let mut vector_5: Vec<i64> = vec![1, 2, 3];

    // let mut vector_7: Vec<&str> = vec![];
    // let mut vector_8: Vec<&str> = Vec::new();

    let vec_eight = vec![1, 2, 3];

    for e in &vec_eight {
        println!("{e}");
    }

    for i in 0..vec_eight.len() {
        println!("{}", vec_eight[i]);
    }

    let vec_population = vec![2, 1, 3];
    let vec_sample = &vec_population[0..1];
    println!("{:?}", vec_sample);

    let mut vec_10 = VecDeque::from(vec!["a", "b", "c"]);

    vec_10.pop_front();
    vec_10.push_front("z");

    println!("data: {:?}", vec_10);
}
