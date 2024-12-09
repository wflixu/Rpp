#![allow(unused_imports)]

use std::fmt::{self, Display, Formatter};
use std::num::NonZeroUsize;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use clap::builder::{TypedValueParser, ValueParser};
use clap::{ArgAction, Args, ColorChoice, Parser, Subcommand, ValueEnum, ValueHint};
use semver::Version;

/// The overall structure of the help.
#[rustfmt::skip]
const HELP_TEMPLATE: &str = "\
Typst {version}

{usage-heading} {usage}

{all-args}{after-help}\
";

/// Adds a list of useful links after the normal help.
#[rustfmt::skip]
const AFTER_HELP: &str = color_print::cstr!("\
<s><u>Resources:</></>
  <s>Tutorial:</>                 https://rppm.app/docs/tutorial/
  <s>Reference documentation:</>  https://rppm.app/docs/reference/
  <s>Templates & Packages:</>     https://rppm.app/universe/
  <s>Forum for questions:</>      https://forum.rppm.app/
");

/// The Typst compiler
#[derive(Debug, Clone, Parser)]
#[clap(
    name = "rppm",
    version = "1.0.0",
    author,
    help_template = HELP_TEMPLATE,
    after_help = AFTER_HELP,
    max_term_width = 80,
)]
pub struct CliArguments {
    /// The command to run
    #[command(subcommand)]
    pub command: Command,

    /// Set when to use color.
    /// auto = use color if a capable terminal is detected
    #[clap(
        long,
        value_name = "WHEN",
        require_equals = true,
        num_args = 0..=1,
        default_value = "auto",
        default_missing_value = "always",
    )]
    pub color: ColorChoice,

    /// Path to a custom CA certificate to use when making network requests.
    #[clap(long = "cert", env = "RPPM_CERT")]
    pub cert: Option<PathBuf>,
}

/// What to do.
#[derive(Debug, Clone, Subcommand)]
#[command()]
pub enum Command {
    /// Compiles an input file into a supported output format
    #[command(visible_alias = "c")]
    Compile(CompileCommand),
}

/// Compiles an input file into a supported output format
#[derive(Debug, Clone, Parser)]
pub struct CompileCommand {
    /// Shared arguments
    #[clap(flatten)]
    pub common: SharedArgs,

    /// Opens the output file with the default viewer or a specific program after
    /// compilation
    ///
    /// Ignored if output is stdout.
    #[arg(long = "open", value_name = "VIEWER")]
    pub open: Option<Option<String>>,

    /// The PPI (pixels per inch) to use for PNG export
    #[arg(long = "ppi", default_value_t = 144.0)]
    pub ppi: f32,
}

/// Common arguments of compile, watch, and query.
#[derive(Debug, Clone, Args)]
pub struct SharedArgs {
    /// Configures the project root (for absolute paths)
    #[clap(long = "root", env = "RPPM_ROOT", value_name = "DIR")]
    pub root: Option<PathBuf>,

    /// Number of parallel jobs spawned during compilation,
    /// defaults to number of CPUs. Setting it to 1 disables parallelism.
    #[clap(long, short)]
    pub jobs: Option<usize>,
}
