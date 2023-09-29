// Rules
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(dead_code)]
// #![allow(unused_must_use)]

// Structs
use switch::Cli;
// Error Handling
use log::trace;
use miette::{MietteHandlerOpts, Result, RgbColors};

/**
The pipelight binary entrypoint.
This main function is the first function to be executed when launching pipelight.
*/
fn main() -> Result<()> {
    trace!("Launch process.");
    make_handler()?;
    Cli::launch()?;
    trace!("Process clean exit.");
    Ok(())
}

/**
The make handler functions is executed right after the main function
to set up a verbose and colorful error/panic handler.
*/
pub fn make_handler() -> Result<()> {
    miette::set_hook(Box::new(|_| {
        Box::new(
            MietteHandlerOpts::new()
                .rgb_colors(RgbColors::Never)
                .color(true)
                .unicode(true)
                .terminal_links(true)
                .context_lines(3)
                .with_cause_chain()
                .build(),
        )
    }))?;
    miette::set_panic_hook();
    Ok(())
}
