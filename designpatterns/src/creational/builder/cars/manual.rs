use crate::creational::builder::components::{CarType, Engine, GpsNavigator, Transmission};

pub struct Manual {
    car_type: CarType,
    seats: u16,
    engine: Engine,
    transmission: Transmission,
    gps_navigator: Option<GpsNavigator>,
}

impl Manual {
    pub fn new(
        car_type: CarType,
        seats: u16,
        engine: Engine,
        transmission: Transmission,
        gps_navigator: Option<GpsNavigator>,
    ) -> Self {
        Self {
            car_type,
            seats,
            engine,
            transmission,
            gps_navigator,
        }
    }
}
