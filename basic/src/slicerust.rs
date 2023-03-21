pub fn sliceRust() {
    let numbers = [12, 16, 8, 3];

    let slice_a = &numbers[0..3];

    let slice_b = &slice_a[1..=2];

    let numbers = [12, 16, 8, 3];
    println!("{}", numbers.len());

    let slice_a = &numbers[0..3];
    println!("{}", slice_a.len());

    let slice_b = &slice_a[1..=2];
    println!("{}", slice_b.len());

    let data = ["a", "b", "c", "d"];

    let sliced_data = &data[1..3];
    println!("{:?}", sliced_data);

    let sliced_data = &data[1..=3];
    println!("{:?}", sliced_data);

    let sliced_data = &data[..3];
    println!("{:?}", sliced_data);

    let sliced_data = &data[..=2];
    println!("{:?}", sliced_data);

    let sliced_data = &data[..];
    println!("{:?}", sliced_data);

    let sliced_data = &data[..];
    println!("{:?}", sliced_data);

    let mut numbers2 = [12, 16, 8, 3];
    println!("===== before =====");
    println!("numbers2 : {:?}", numbers2);

    let slice_e = &mut numbers2[..=2];
    slice_e[1] = 99;

    println!("===== after =====");
    println!("slice_e  : {:?}", slice_e);
    println!("numbers2 : {:?}", numbers2);

    let scores1 = [7, 8, 9];

    for score in &scores1[..] {
        print!("{:?} ", score);
    }

    let mut scores2 = [7, 8, 9];
    println!("(before) scores2 : {:?}", scores2);

    let slice_f = &mut scores2[..];

    for score in &mut slice_f[..] {
        *score += 1;
    }

    println!("(after)  scores2 : {:?}", scores2);
}
