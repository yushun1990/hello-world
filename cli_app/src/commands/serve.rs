use clap::{Arg, ArgMatches, Command, value_parser};

pub const COMMAND_NAME: &'static str = "serve";

pub fn configure() -> Command {
    Command::new(COMMAND_NAME).about("Start HTTP Server").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("TCP port to listen on")
            .default_value("8080")
            .value_parser(value_parser!(u16)),
    )
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    let port: u16 = *matches.get_one("port").unwrap();

    println!("TBD: Start web server on port: {}", port);

    Ok(())
}
