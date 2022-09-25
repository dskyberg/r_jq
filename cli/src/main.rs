use std::{
    env,
    io::{stdin, Read},
};

use r_jq::{jq, JQError};

fn main() -> Result<(), JQError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(JQError::GeneralError(
            "Error: No query provided".to_string(),
        ));
    }

    let mut buffer = Vec::new();
    let mut stdin = stdin();

    // Read and parse the json input.
    let _ = stdin.read_to_end(&mut buffer)?;

    let results = jq(&buffer, &args[1])?;
    if let Some(values) = results {
        for value in values {
            let output = r_jq::serde_json::to_string_pretty(&value)?;
            println!("{}", &output);
        }
    }
    Ok(())
}
