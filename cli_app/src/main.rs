use clap::{Arg, Command};
use cli_app::{commands, settings};

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let mut command = Command::new("Sample CLI application").arg(
        Arg::new("config")
            .short('c')
            .long("config")
            .help("Configuration file location"),
    );
    command = commands::configure(command);

    let matches = command.get_matches();
    let config_location = matches
        .get_one::<String>("config")
        .map(|s| Some(s.as_str()))
        .unwrap_or(None);

    let settings = settings::Settings::new(config_location, "APP")?;

    println!(
        "db url: {}",
        settings
            .database
            .url
            .unwrap_or("missing databse url".to_string())
    );

    println!(
        "log level: {}",
        settings.logging.log_level.unwrap_or("info".to_string())
    );

    commands::handle(&matches)?;

    Ok(())
}
