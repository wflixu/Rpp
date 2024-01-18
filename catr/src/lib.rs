use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;
pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
    pub number_lines: bool,
    pub number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Luke")
        .about("Luke Catr")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .required(true)
                .help("input file name")
                .min_values(1),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .help("是否显示行号")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .help("是否显示空行行号")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}
