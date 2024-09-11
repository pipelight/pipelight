#![feature(async_closure)]

// Clap completion script generation
use clap_complete::{generate_to, Shell};
use std::env;
// Filesystem manipulation
use std::fs;
use std::path::Path;
// Error Handling
use miette::{IntoDiagnostic, Result};

include!("src/lib.rs");
pub use crate::types::Cli;

/**
Build cli and generate autocompletion scripts
*/
fn main() -> Result<()> {
    // Standard outdir
    // let outdir = match env::var_os("OUT_DIR") {
    //     None => return Ok(()),
    //     Some(outdir) => outdir,
    // };

    // Practical outdir
    let outdir = Path::new("../autocompletion/");

    fs::create_dir_all(outdir).into_diagnostic()?;

    let mut cmd = Cli::build()?;
    let name = cmd.get_name().to_string();
    let shells = vec![Shell::Bash, Shell::Zsh, Shell::Fish];
    for shell in shells {
        let path = generate_to(
            shell,
            &mut cmd, // We need to specify what generator to use
            name.clone(),
            outdir, // We need to specify where to write to
        )
        .into_diagnostic()?;
        println!("cargo:warning=completion file is generated: {path:?}");
    }
    Ok(())
}
