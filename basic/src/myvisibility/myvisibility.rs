pub mod outher_mod {
    pub mod inner_mod {
        pub(in crate::myvisibility::myvisibility::outher_mod) fn say_hello() {
            println!("hello rust");
        }
    }

    pub fn do_something() {
        inner_mod::say_hello();
    }
}
