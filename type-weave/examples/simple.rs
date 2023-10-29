use type_weave::prelude::*;

#[derive(Weave, Debug)]
#[layer(Config)]
struct Config {
    fast: bool,
    message: Option<&'static str>,
}

fn main() {
    let layer1 = Config {
        fast: true,
        message: None,
    };

    let layer2 = Config {
        fast: false,
        message: Some("layer 2 stuff"),
    };

    let config: Config = layer1.layer(layer2);
    println!("{:#?}", config);
}
