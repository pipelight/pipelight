use crate::types::{Command, Event, Parallel, Pipeline, Step, StepOrParallel, Trigger};
use colored::Colorize;
use exec::types::{Statuable, Status};
use log::{debug, info, warn};
use std::error::Error;

pub mod characters;
pub mod composables;
use super::tree::characters::Characters;

static INDENT: &str = "   ";

pub fn make_branch(index: usize, vec_length: usize, level: usize) -> String {
    let leaf: String;
    let mut indent = "".to_owned();

    if index < vec_length {
        let mut i = 0;
        while i < level {
            i = i + 1;
            indent.push_str(&format!("{}", Characters::unicode().vbar));
            indent.push_str(&INDENT);
        }
        // indent.push_str(&INDENT.repeat(level));
        leaf = format!(
            "{indent:}{}\n{indent:}{}{}",
            Characters::unicode().vbar,
            Characters::unicode().lcross,
            Characters::unicode().hbar,
            indent = indent,
        );
    } else {
        let mut i = 0;
        while i < level {
            i = i + 1;
            // indent.push_str(&format!("{}", Characters::unicode().vbar));
            indent.push_str("");
            indent.push_str(&INDENT);
        }
        // indent.push_str(&INDENT.repeat(level));
        leaf = format!(
            "{indent:}{}\n{indent:}{}{}",
            Characters::unicode().vbar,
            Characters::unicode().lbot,
            Characters::unicode().hbar,
            indent = indent,
        );
    }
    return leaf;
}
pub trait Tree {
    /// Return a tree structure
    fn make_stateless_tree(&self, level: usize) -> String;
    fn make_statefull_tree(&self, level: usize) -> String;
}

impl Tree for Pipeline {
    fn make_statefull_tree(&self, level: usize) -> String {
        let mut printable: String = "".to_owned();

        // Make root
        let root = format!("pipeline: {}\n", &self.name);
        printable.push_str(&root);

        // Make sub branch
        let vec_length = &self.steps.len() - 1;
        for (i, e) in self.steps.iter().enumerate() {
            match e {
                StepOrParallel::Step(res) => {
                    let leafed = format!(
                        "{}{}",
                        make_branch(i, vec_length, level),
                        &res.make_statefull_tree(level),
                    );
                    printable.push_str(&leafed);
                }

                StepOrParallel::Parallel(res) => {
                    let leafed = format!("{}", &res.make_statefull_tree(level),);
                    printable.push_str(&leafed);
                }
            }
        }
        return printable;
    }
    fn make_stateless_tree(&self, level: usize) -> String {
        let mut printable: String = "".to_owned();

        // Make root
        let root = format!("pipeline: {}\n", &self.name);
        printable.push_str(&root);

        // Make branch
        let vec_length = &self.steps.len() - 1;
        for (i, e) in self.steps.iter().enumerate() {
            match e {
                StepOrParallel::Step(res) => {
                    let leafed = format!(
                        "{}{}",
                        make_branch(i, vec_length, level),
                        &res.make_statefull_tree(level),
                    );
                    printable.push_str(&leafed);
                }

                StepOrParallel::Parallel(res) => {
                    let leafed = format!("{}", &res.make_stateless_tree(level),);
                    printable.push_str(&leafed);
                }
            }
            match e.get_status() {
                None => {}
                Some(Status::Failed) => {
                    break;
                }
                _ => {}
            }
        }
        return printable;
    }
}
impl Tree for StepOrParallel {
    fn make_statefull_tree(&self, level: usize) -> String {
        let printable: String;
        match self {
            StepOrParallel::Step(res) => printable = res.make_statefull_tree(level),
            StepOrParallel::Parallel(res) => printable = res.make_statefull_tree(level),
        }
        return printable;
    }
    fn make_stateless_tree(&self, level: usize) -> String {
        let printable: String;
        match self {
            StepOrParallel::Step(res) => printable = res.make_stateless_tree(level),
            StepOrParallel::Parallel(res) => printable = res.make_stateless_tree(level),
        }
        return printable;
    }
}
impl Tree for Parallel {
    fn make_statefull_tree(&self, level: usize) -> String {
        let mut level = level;
        let mut printable: String = "".to_owned();

        // Make branch
        let vec_length = &self.steps.len() - 1;
        for (i, e) in self.steps.iter().enumerate() {
            let leafed = format!(
                "{}{}",
                make_branch(i, vec_length, level),
                &e.make_statefull_tree(level),
            );
            printable.push_str(&leafed);
        }
        return printable;
    }
    fn make_stateless_tree(&self, level: usize) -> String {
        let mut level = level;
        let mut printable: String = "".to_owned();
        // Make branch
        let vec_length = &self.steps.len() - 1;
        for (i, e) in self.steps.iter().enumerate() {
            let leafed = format!(
                "{}{}",
                make_branch(i, vec_length, level),
                &e.make_statefull_tree(level),
            );
            printable.push_str(&leafed);
        }
        return printable;
    }
}

impl Tree for Step {
    fn make_statefull_tree(&self, level: usize) -> String {
        let mut level = level;
        let indent = INDENT.repeat(level);
        let mut printable: String = "".to_owned();

        // Make root
        let root = format!(" step: {}\n", &self.name);
        printable.push_str(&root);

        // Make sub branch
        level = level + 1;
        let vec_length = &self.commands.len() - 1;
        for (i, e) in self.commands.iter().enumerate() {
            let leafed = format!(
                "{}{}",
                make_branch(i, vec_length, level),
                &e.make_statefull_tree(level),
            );
            printable.push_str(&leafed);
        }

        return printable;
    }
    fn make_stateless_tree(&self, level: usize) -> String {
        let mut level = level;
        let indent = INDENT.repeat(level);
        let mut printable: String = "".to_owned();

        // Make root
        let root = format!(" step: {}\n", &self.name);
        printable.push_str(&root);

        // Make sub branch
        level = level + 1;
        let vec_length = &self.commands.len() - 1;
        for (i, e) in self.commands.iter().enumerate() {
            let leafed = format!(
                "{}{}",
                make_branch(i, vec_length, level),
                &e.make_stateless_tree(level),
            );
            printable.push_str(&leafed);
        }

        return printable;
    }
}
impl Tree for Command {
    fn make_statefull_tree(&self, level: usize) -> String {
        let mut level = level;
        // let mut indent = INDENT.repeat(level);
        let mut indent = "".to_owned();

        let mut printable: String = "".to_owned();

        let root: String;
        let cmd_with_leaf: String;
        match self.status {
            Some(Status::Started) => {
                root = format!(" {}\n", &self.stdin);
                printable.push_str(&root);
            }
            Some(Status::Running) => {
                root = format!(" {}\n", &self.stdin.green());
                printable.push_str(&root);
            }
            Some(Status::Succeeded) => {
                root = format!(" {}\n", &self.stdin.blue());
                printable.push_str(&root);

                // Sub branch
                level = level + 1;
                // indent = INDENT.repeat(level);
                let mut i = 0;
                while i < level {
                    i = i + 1;
                    indent.push_str(&format!("{}", Characters::unicode().vbar));
                    indent.push_str(&INDENT);
                }

                if self.output.clone().unwrap().stdout.is_some() {
                    let mut out = self.output.clone().unwrap().stdout.unwrap();

                    // Indent output
                    out = out.replace("\n", &format!("\n{indent:} ", indent = indent));
                    out.push_str("\n");

                    let leafed = format!("{}{}", make_branch(0, 0, level), out);
                    printable.push_str(&leafed);
                }
            }
            Some(Status::Failed) => {
                root = format!(" {}\n", &self.stdin.red());
                printable.push_str(&root);

                // Sub branch
                level = level + 1;
                indent = INDENT.repeat(level);
                if self.output.clone().unwrap().stderr.is_some() {
                    let mut out = self.output.clone().unwrap().stderr.unwrap();

                    // Indent output
                    out = out.replace("\n", &format!("\n{indent:} ", indent = indent));
                    out.push_str("\n");

                    let leafed = format!("{}{}", make_branch(0, 0, level), out);
                    printable.push_str(&leafed);
                }
            }
            Some(Status::Aborted) => {
                root = format!(" {}\n", &self.stdin.yellow());
                printable.push_str(&root);
            }
            None => {}
        }
        return printable;
    }
    fn make_stateless_tree(&self, level: usize) -> String {
        let mut printable: String = "".to_owned();

        let root = format!(" {}\n", &self.stdin);
        printable.push_str(&root);

        return printable;
    }
}
