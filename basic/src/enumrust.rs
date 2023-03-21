enum Food {
    PenyetanTerangBulan,
    PizzaNanas,
    EsKrimIkanMujaer,
    MiGorengKuah,
    MakananLainya(String),
    MieSetan {
        level_pedas: i32,
        pakek_piring: bool,
    },
}

fn myenum() {
    let makanan_favorit: Food = Food::MieSetan {
        level_pedas: 5,
        pakek_piring: false,
    };
    let nasi_goren = String::from("nasi goreng");

    match makanan_favorit {
        Food::PenyetanTerangBulan => {
            println!("your food taste is quite ... unique");
        }
        Food::PizzaNanas => {
            println!("it's morally wrong to have pineaple on top of pizza");
        }
        Food::EsKrimIkanMujaer => {
            println!("I don't know what to say");
        }
        Food::MiGorengKuah => {
            println!("sometimes people do eat this, but it's ok");
        }
        Food::MakananLainya(m) => {
            println!("do you like {m}? nice taste!");
        }

        Food::MieSetan {
            level_pedas,
            pakek_piring,
        } => {
            if level_pedas > 3 {
                println!("mie setan lvl {} is too much!", level_pedas);
            } else {
                println!("mie setan lvl {} is perfect", level_pedas);
            }

            if !pakek_piring {
                println!("how are you going to eat the food without a plate, huh?");
            }
        }
        _ => {
            println!("never heard about that food");
        }
    }
}
