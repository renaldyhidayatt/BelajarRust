pub mod outer_mod_one {

    pub mod inner_mod {

        pub(crate) fn say_hello() {
            println!("hello rust")
        }
    }

    pub fn do_something() {
        inner_mod::say_hello();
    }
}

pub mod outer_mod_two {

    pub fn do_something() {
        crate::myvisi::myvisi::outer_mod_one::inner_mod::say_hello();
    }
}
