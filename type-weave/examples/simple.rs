use type_weave::prelude::*;

#[derive(Weave, Debug)]
struct Config {
    release: bool,
    package: Option<&'static str>,
}

fn main() {
    let c1 = Config {
        release: false,
        package: Some("this"),
    };
    let c2 = Config {
        release: true,
        package: None,
    };
    let c3 = Config {
        release: false,
        package: Some("that"),
    };

    let config = c1.under(c2).under(c3);
    // should print: `Config { release: true, package: Some("that") }`
    println!("{:?}", config);
}
