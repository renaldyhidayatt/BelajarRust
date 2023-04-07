use super::adaptee::SpecificTarget;
use super::adapter::TargetAdapter;
use super::target::{OrdinaryTarget, Target};

fn call(target: impl Target) {
    println!("'{}'", target.request());
}

pub fn running() {
    let target = OrdinaryTarget;

    print!("A compatible target can be directly called: ");
    call(target);

    let adaptee = SpecificTarget;

    println!(
        "Adaptee is incompatible with client: '{}'",
        adaptee.specific_request()
    );

    let adapter = TargetAdapter::new(adaptee);

    print!("But with adapter client can call its method: ");
    call(adapter);
}
