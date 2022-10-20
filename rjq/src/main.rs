use std::io::{stdin, Read};

use clap::Parser;
use pretty_print::*;
use r_jq::jq;
mod pretty_print;
/*
  --tab            use tabs for indentation;
  --arg a v        set variable $a to value <v>;
  --argjson a v    set variable $a to JSON value <v>;
  --slurpfile a f  set variable $a to an array of JSON texts read from <f>;
  --rawfile a f    set variable $a to a string consisting of the contents of <f>;
  --args           remaining arguments are string arguments, not files;
  --jsonargs       remaining arguments are JSON arguments, not files;
*/
#[derive(Parser)]
#[clap(author, version, long_about = None)]
#[clap(about("commandline JSON processor "))]
pub struct Cli {
    /// compact instead of pretty-printed output;
    #[clap(short, action, default_value_t = false)]
    compact: bool,

    /// use `null` as the single input value;
    #[clap(short, action, default_value_t = false)]
    null: bool,

    /// set the exit status code based on the output;
    #[clap(short, action, default_value_t = false)]
    exit: bool,

    /// read (slurp) all inputs into an array; apply filter to it;
    #[clap(short, action, default_value_t = false)]
    slurp: bool,

    /// output raw strings, not JSON texts;
    #[clap(short, action, default_value_t = false)]
    raw_out: bool,

    /// read raw strings, not JSON texts;
    #[clap(short = 'R', action, default_value_t = false)]
    raw_in: bool,

    /// colorize JSON;
    #[clap(short = 'C', action, default_value_t = true)]
    colorize: bool,

    /// monochrome (don't colorize JSON);
    #[clap(short = 'M', action, default_value_t = false)]
    monochrome: bool,

    /// sort keys of objects on output;
    #[clap(short = 'S', action, default_value_t = false)]
    sort: bool,

    /// display the curren verion
    #[clap(short, long)]
    version: bool,

    /// JQ style query statement
    #[clap(value_parser)]
    query: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut buffer = Vec::new();
    let mut stdin = stdin();

    // Read and parse the json input.
    let _ = stdin.read_to_end(&mut buffer)?;

    let results = jq(&buffer, &cli.query)?;
    let mut pretty = PrettyPrint::new();
    for result in results {
        //let output = r_jq::serde_json::to_string_pretty(&result)?;
        let _ = pretty.print(&result, true);
        println!();
    }
    Ok(())
}
