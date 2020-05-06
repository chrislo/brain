use clap::{App, Arg};

pub fn parse() -> clap::ArgMatches {
    App::new("brain")
        .arg(
            Arg::with_name("controller")
                .long("controller")
                .default_value("atom"),
        )
        .get_matches()
}

pub fn controller_addr() -> String {
    let config = parse();
    config.value_of("controller").unwrap().to_string()
}
