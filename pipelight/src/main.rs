#![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(dead_code)]
#![allow(unused_must_use)]

use cli::case::Client;

// Error Handling
use miette::{MietteHandlerOpts, Result, RgbColors};

fn main() -> Result<()> {
    make_handler()?;
    Client::launch()?;
    Ok(())
}
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
