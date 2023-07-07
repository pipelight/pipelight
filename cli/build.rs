use clap_complete::{generate_to, shells::Shell};
use std::env;
// use std::fs;
// use std::path::Path;

// Error Handling
use miette::{IntoDiagnostic, Result};

include!("src/lib.rs");
use crate::case::Client;

fn main() -> Result<()> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    // Practical outdir
    // let outdir = Path::new("../autocompletion/");
    // fs::create_dir_all(outdir).into_diagnostic()?;

    //Build client and generate autocompletion scripts
    let mut cmd = Client::build()?;
    for shell in vec![Shell::Bash, Shell::Zsh, Shell::Fish, Shell::Elvish] {
        let path = generate_to(
            shell,
            &mut cmd,       // We need to specify what generator to use
            "pipelight",    // We need to specify the bin name manually
            outdir.clone(), // We need to specify where to write to
        )
        .into_diagnostic()?;
        println!("cargo:warning=completion file is generated: {path:?}");
    }
    Ok(())
}
