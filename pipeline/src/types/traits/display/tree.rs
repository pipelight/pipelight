use super::characters::Characters;
use crate::types::{Command, Event, Parallel, Pipeline, Step, StepOrParallel, Trigger};
use std::error::Error;

pub trait Tree<T> {
    /// Return a tree structure
    fn make_tree(&self) -> Result<(), Box<dyn Error>>;
}

static INDENT: &str = " ";

impl Tree<Pipeline> for Pipeline {
    fn make_tree(&self) -> Result<(), Box<dyn Error>> {
        let indent = INDENT.repeat(4);
        let has_status = self.status.is_some();

        let mut leaf = format!(
            "{indent:}{}\n{indent:}{}{}",
            Characters::unicode().mtop,
            Characters::unicode().lcross,
            Characters::unicode().hbar,
            indent = indent,
        );
        println!("{indent:}pipeline: {}", &self.name, indent = indent);

        let steps_length = &self.steps.len() - 1;
        for (i, step) in self.steps.iter().enumerate() {
            let indent = INDENT.repeat(8);
            if 0 <= i && i < steps_length {
                leaf = format!(
                    "{indent:}{}\n{indent:}{}{}",
                    Characters::unicode().vbar,
                    Characters::unicode().lcross,
                    Characters::unicode().hbar,
                    indent = indent,
                );
            }
            if i == steps_length {
                leaf = format!(
                    "{indent:}{}\n{indent:}{}{}",
                    Characters::unicode().vbar,
                    Characters::unicode().lbot,
                    Characters::unicode().hbar,
                    indent = indent,
                );
            }
            println!("{}{}", leaf, step);
        }

        Ok(())
    }
}
impl Tree<Step> for Step {
    fn make_tree(&self) -> Result<(), Box<dyn Error>> {
        let indent = INDENT.repeat(0);
        let has_status = self.status.is_some();
        let leaf = Characters::unicode().hbar;

        let mut leaf = format!(
            "{indent:}{}\n{indent:}{}{}",
            Characters::unicode().mtop,
            Characters::unicode().lcross,
            Characters::unicode().hbar,
            indent = indent,
        );
        println!("{indent:}step: {}", &self.name, indent = indent);

        let cmds_length = &self.commands.len() - 1;
        for (i, cmd) in self.commands.iter().enumerate() {
            let indent = INDENT.repeat(4);
            if 0 <= i && i < cmds_length {
                leaf = format!(
                    "{indent:}{}\n{indent:}{}{}",
                    Characters::unicode().vbar,
                    Characters::unicode().lcross,
                    Characters::unicode().hbar,
                    indent = indent,
                );
            }
            if i == cmds_length {
                leaf = format!(
                    "{indent:}{}\n{indent:}{}{}",
                    Characters::unicode().vbar,
                    Characters::unicode().lbot,
                    Characters::unicode().hbar,
                    indent = indent,
                );
            }
            println!("{}{}", leaf, cmd);
        }
        Ok(())
    }
}
