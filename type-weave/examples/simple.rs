use type_weave::prelude::*;

#[derive(Weave, Debug)]
#[layer(ConfigA, ConfigB)]
struct ConfigA {
    fast: bool,
    message: Option<&'static str>,
}

#[derive(Debug)]
struct ConfigB {
    fast: bool,
    message: Option<&'static str>,
}

fn main() {
    let layer1 = ConfigA {
        fast: true,
        message: Some("Will not be seen!"),
    };

    let layer2 = ConfigB {
        fast: false,
        message: Some("Layer 2 stuff"),
    };

    let config: ConfigA = layer1.layer(layer2);
    println!("{:#?}", config);
}
