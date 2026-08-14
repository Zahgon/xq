use std::{
    io::{stdin, stdout, BufRead, Read, Write},
    path::PathBuf,
};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use cli::input::Input;
use is_terminal::IsTerminal;
use xq::{module_loader::PreludeLoader, run_query, InputError, Value};

use crate::cli::input::Tied;

mod cli;

#[derive(Parser, Debug)]
#[clap(author, about, version)]
#[clap(long_version(option_env!("LONG_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))))]
struct Cli {
    
    #[clap(default_value = ".")]
    query: String,

    #[clap(
        name = "file",
        short = 'f',
        long = "from-file",
        conflicts_with = "query",
        value_hint = clap::ValueHint::FilePath
    )]
    query_file: Option<PathBuf>,

    #[clap(flatten)]
    input_format: InputFormatArg,

    #[clap(flatten)]
    output_format: OutputFormatArg,

    #[clap(flatten)]
    verbosity: Verbosity,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, clap::ValueEnum)]
enum SerializationFormat {
    #[default]
    Json,
    Yaml,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, clap::Args)]
struct InputFormatArg {
    
    #[arg(long, value_enum, default_value_t, group = "input-format")]
    input_format: SerializationFormat,

    #[arg(long, group = "input-format")]
    json_input: bool,

    #[arg(long, group = "input-format")]
    yaml_input: bool,

    #[arg(short = 'R', long, group = "input-format")]
    raw_input: bool,

    #[arg(short, long)]
    null_input: bool,

    #[arg(short, long)]
    slurp: bool,
}

impl InputFormatArg {
    fn get(self) -> SerializationFormat { panic!("STUB: not implemented") }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, clap::Args)]
struct OutputFormatArg {
    
    #[arg(long, value_enum, default_value_t, group = "output-format")]
    output_format: SerializationFormat,

    #[arg(long, group = "output-format")]
    json_output: bool,

    #[arg(long, group = "output-format")]
    yaml_output: bool,

    #[clap(short, long, conflicts_with = "output-format")]
    raw_output: bool,

    #[clap(short, long, conflicts_with = "output-format")]
    compact_output: bool,

    #[clap(short = 'C', long, group = "output-color")]
    color_output: bool,

    #[clap(short = 'M', long, group = "output-color")]
    monochrome_output: bool,
}

impl OutputFormatArg {
    fn get(self) -> SerializationFormat { panic!("STUB: not implemented") }
}

fn init_log(verbosity: &Verbosity) -> Result<()> { panic!("STUB: not implemented") }

fn get_json_style() -> colored_json::Styler { panic!("STUB: not implemented") }

fn run_with_input(cli: Cli, input: impl Input) -> Result<()> { panic!("STUB: not implemented") }

fn run_with_maybe_null_input(cli: Cli, input: impl Input) -> Result<()> { panic!("STUB: not implemented") }

fn run_with_maybe_slurp_null_input<I: Iterator<Item = Result<Value, InputError>>>(
    args: Cli,
    input: Tied<I>,
) -> Result<()> { panic!("STUB: not implemented") }

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();
    init_log(&cli.verbosity)?;
    log::debug!("Parsed argument: {cli:?}");

    let stdin = stdin();
    let mut locked = stdin.lock();

    if cli.input_format.raw_input {
        if cli.input_format.slurp {
            let mut input = String::new();
            locked.read_to_string(&mut input)?;
            run_with_maybe_null_input(cli, Tied::new(std::iter::once(Ok(Value::from(input)))))
        } else {
            let input = locked
                .lines()
                .map(|l| l.map(Value::from).map_err(InputError::new));
            run_with_maybe_null_input(cli, Tied::new(input))
        }
    } else {
        match cli.input_format.get() {
            SerializationFormat::Json => {
                let input = serde_json::de::Deserializer::from_reader(locked)
                    .into_iter::<Value>()
                    .map(|r| r.map_err(InputError::new));
                run_with_maybe_slurp_null_input(cli, Tied::new(input))
            }
            SerializationFormat::Yaml => {
                use serde::Deserialize;
                let input = serde_yaml::Deserializer::from_reader(locked)
                    .map(Value::deserialize)
                    .map(|r| r.map_err(InputError::new));
                run_with_maybe_slurp_null_input(cli, Tied::new(input))
            }
        }
    }
}
