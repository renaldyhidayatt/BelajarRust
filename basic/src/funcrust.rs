pub fn calculate_box_volume1(width: i32, height: i32, length: i32) -> i32 {
    let volume = width * height * length;

    return volume;
}

pub fn greet_custom_message(name: &str, message: &str) {
    println!("hi {name}, {message}");
}

pub fn get_score_message(score: f32) -> &'static str {
    if score == 100.0 {
        return "you got a perfect score!";
    }

    if score > 76.0 {
        return "mantap";
    }

    return "uaha";
}
