#[derive(Debug)]
pub struct Car {
    brand: String,
    model: String,
    manufacture_year: i32,
}

impl Car {
    pub fn new(brand: String, model: String) -> Self {
        Self {
            brand: brand,
            model: model,
            manufacture_year: 0,
        }
    }

    pub fn info(&self) -> String {
        if self.manufacture_year == 0 {
            format!("{} model {}", self.brand, self.model)
        } else {
            format!(
                "{} model {}, manufactured at {}",
                self.brand, self.model, self.manufacture_year
            )
        }
    }

    pub fn congratulate(&self, name: String) {
        println!("hello {}", name);
        println!("congrats with your new car {}", self.info());
        println!("vroooom vroooooooommmmm!");
    }

    pub fn set_manufacture_year(&mut self, year: i32) {
        self.manufacture_year = year
    }
}

pub fn mydota() {
    let mut car = Car::new(
        String::from("Mercedes-Benz"),
        String::from("Vision Gran Turismo"),
    );

    println!("car: {:?}", car);

    let info = car.info();
    println!("info: {:?}", info);

    car.set_manufacture_year(2013);

    let detailed_info = car.info();

    println!("detailed info: {:?}", detailed_info);
}
