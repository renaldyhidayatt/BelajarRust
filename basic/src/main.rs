mod construst;
mod helloworld;
mod kondisi;
mod operator;
mod perulangan;
mod perulanganloop;
mod stringrust;
mod tipedata;

fn main() {
    perulangan::perulangan();
    operator::operator();

    perulanganloop::loopingloop();

    kondisi::kondisi();

    println!("{}", construst::NUMBER);
    construst::construst();

    tipedata::tipedata();
    stringrust::stringrust();
}
