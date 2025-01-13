use clap::{CommandFactory, Parser};
use clap_complete::{Generator, Shell, generate};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The file or path to format
    #[arg(default_value = ".")]
    pub file: Vec<String>,

    #[clap(long, short, default_value = "false")]
    pub stdin: bool,

    /// Generate shell completion script
    #[arg(long, value_enum)]
    pub gen_completion: Option<Shell>,
}

pub fn print_completions<G: Generator>(gen: G) {
    let mut cmd = Args::command();

    generate(gen, &mut cmd, "pestfmt", &mut std::io::stdout());
}
