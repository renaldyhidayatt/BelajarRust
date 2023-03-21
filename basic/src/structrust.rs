struct User {
    name: String,
    sign_in_count: u64,
    affliation: Vec<String>,
    actice: bool,
}

struct Car {
    brand: String,
    model: String,
}

struct Point {
    x: f32,
    y: f32,
}

pub fn mystruct() {
    let user_one = User {
        name: "Rusty".to_owned(),
        sign_in_count: 30,
        affliation: vec![
            "dota".to_owned(),
            "dota".to_owned(),
            "dota".to_owned(),
            "dota".to_owned(),
        ],
        actice: true,
    };

    println!("name: {}", user_one.name);
    println!("sign-in count: {}", user_one.sign_in_count);
    println!("affliation: {:?}", user_one.affliation);
    println!("is active? :{}", user_one.actice);

    let mut user_two: User = User {
        name: "dota".to_owned(),
        sign_in_count: 30,
        affliation: vec!["dota".to_owned(), "dota".to_owned(), "dota".to_owned()],

        actice: true,
    };

    user_two.name = String::from("Renaldy Hidayat");
    user_two.affliation.pop();
    user_two.actice = true;

    println!("name: {}", user_two.name);
    println!("sign-in count: {}", user_two.sign_in_count);
    println!("affliation: {:?}", user_two.affliation);
    println!("is active? :{}", user_two.actice);

    let car_one = Car {
        brand: String::from("Toyota"),
        model: String::from("Rusty"),
    };

    let car_two: Car = Car {
        brand: String::from("BMW"),
        model: String::from("M3 GTR"),
    };

    let mut car_three: Car;

    car_three = Car {
        brand: String::from("Audi"),
        model: String::from("Le Mans Quattro"),
    };

    println!("{} {}", car_three.brand, car_three.model);

    let mut car_four: Car;
    car_four = Car {
        brand: String::from("Audi Brand"),
        ..car_three
    };
    println!("{} {}", car_four.brand, car_four.model);

    let point_one = Point { x: 3.14, y: 8.0 };

    let Point { x: x_one, y: y_one } = point_one;
    println!("x_one: {}", x_one);
    println!("y_one: {}", y_one);
}
