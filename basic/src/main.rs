mod aliasrust;
mod arrayrust;
mod associatedfunc;
mod block;
mod construst;
mod enumrust;
mod funcrust;
mod helloworld;
mod kondisi;
mod method;
mod my_number;
mod myvisi;
mod myvisibility;
mod operator;
mod perulangan;
mod perulanganloop;
mod pointer;
mod slicerust;
mod stringrust;
mod structrust;
mod tipedata;
mod tuplerust;
mod vectorrust;
mod visibility_privacy;

use block::blockrust;
use pointer::pointer;
use rand::Rng;
use visibility_privacy::messaging::say_hello;

use my_number::{conversion_utility::string_to_number, is_odd_number};
use structrust::mystruct;

use myvisibility::myvisibility::outher_mod::do_something;

fn generate_random_number() -> i32 {
    return rand::thread_rng().gen_range(0..100);
}

fn main() {
    for i in 0..5 {
        println!("random number {}: {}", i, generate_random_number());
    }
    blockrust();

    pointer();

    string_to_number("dota".to_owned());

    visibility_privacy::messaging::say_hello();

    do_something();

    is_odd_number(83.to_owned());

    say_hello();

    mystruct();

    // arrayrust::arrayRust();

    // perulangan::perulangan();
    // operator::operator();

    // calculate_box_volume1(10, 20, 30);

    // greet_custom_message("Alucard", "Welcome to feed");
    // get_score_message(300.0);

    // perulanganloop::loopingloop();

    // vectorRust();

    // kondisi::kondisi();

    // println!("{}", construst::NUMBER);
    // construst::construst();

    // tipedata::tipedata();
    // stringrust::stringrust();
}
