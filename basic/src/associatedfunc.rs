#[derive(Debug)]
struct LegoSet {
    code: i32,
    name: String,
    category: String,
    age_minimum: i32,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

impl LegoSet {
    fn new(code: i32, name: String, category: String, age_minimum: i32) -> LegoSet {
        return LegoSet {
            code: code,
            name: name,
            category: category,
            age_minimum: age_minimum,
        };
    }

    fn what_is_lego() {
        println!("Lego adalah");
    }
}

impl Color {
    fn red() -> Self {
        Self(255, 0, 0)
    }

    fn green() -> Self {
        Self(0, 255, 0)
    }

    fn blue() -> Self {
        Self(0, 0, 255)
    }
}

pub fn associatedfunc() {
    LegoSet::what_is_lego();
    let rought_terrain_cran = LegoSet {
        code: 100,
        name: "Terrain".to_owned(),
        category: "kentang".to_owned(),
        age_minimum: 11,
    };

    println!("{:#?}", rought_terrain_cran);

    let xtreme_offorader = LegoSet::new(100, "dota".to_owned(), "kentang".to_owned(), 11);

    println!("{:#?}", xtreme_offorader);

    let red = Color::red();
    let green = Color::green();
    let blue = Color::blue();

    println!("{:#?} {:#?} {:#?}", red, green, blue);
}
