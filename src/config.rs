pub struct Config {
    pub prefix: String,
    pub sufix: String,
    pub auto_push: bool,
    pub fetch: bool,
    pub major: bool,
    pub minor: bool,
    pub patch: bool,
}

impl Config {
    pub fn new() -> Config {
        Config {
            prefix: String::from(""),
            sufix: String::from(""),
            auto_push: false,
            fetch: false,
            major: false,
            minor: false,
            patch: false,
        }
    }
}
