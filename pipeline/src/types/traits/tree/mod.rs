use crate::types::{Command, Event, Parallel, Pipeline, Step, StepOrParallel, Trigger};
use colored::Colorize;
use exec::types::{Statuable, Status};
use log::{debug, info, warn};
use std::error::Error;
use std::fmt::Display;

pub mod characters;
pub mod def;
pub mod statuable;
use super::tree::characters::Characters;

static INDENT: &str = "  ";

// Logic
//
// Pass "indent" variable to every function.
// It is a string suffix that creates the tree.
//

/// Modifier for indent_suffix
/// Add a depth level to the indent_suffix, and return the result
/// Every subsrtuct output must be suffixed to print a working tree
// pub fn add_level(indent: &mut String, index: usize, vec_length: usize) -> String {
pub fn add_level(indent: &String) -> String {
    let leaf: String = format!("{}{INDENT:}", Characters::unicode().vbar, INDENT = INDENT,);
    let mut new_indent = indent.to_owned();
    new_indent.push_str(&leaf);
    return new_indent;
}
/// Add a leaf to suffix T from inside [T] of nth element
pub fn make_branch(suffix: String, index: usize, vec_length: usize) -> String {
    let leaf: String;
    if index < vec_length {
        leaf = format!(
            "{suffix:}{}\n{suffix:}{}{}",
            Characters::unicode().vbar,
            Characters::unicode().lcross,
            Characters::unicode().hbar,
            suffix = suffix,
        );
    } else {
        leaf = format!(
            "{suffix:}{}\n{suffix:}{}{}",
            Characters::unicode().vbar,
            Characters::unicode().lbot,
            Characters::unicode().hbar,
            suffix = suffix,
        );
    }
    // suffix.push_str(&leaf);
    return leaf;
    // return suffix.to_owned();
}

pub trait Tree {
    /// Draw the struct in a tree view
    fn make_stateless_tree(&self, indent: &String) -> String;
    /// Draw the struct in a tree view and add status to branches
    fn make_statefull_tree(&self, indent: &String) -> String;
}

impl Tree for Pipeline {
    fn make_statefull_tree(&self, indent: &String) -> String {
        let mut printable: String = "".to_owned();

        // Make root
        let root = format!("pipeline: {}\n", &self.name);
        printable.push_str(&root);

        // Make sub branch
        let vec_length = &self.steps.len() - 1;
        // indent = add_level(indent)

        for (i, e) in self.steps.iter().enumerate() {
            match e {
                StepOrParallel::Step(res) => {
                    let leafed = format!(
                        "{}{}",
                        make_branch(indent.clone(), i, vec_length),
                        &res.make_statefull_tree(indent),
                    );
                    printable.push_str(&leafed);
                }
                StepOrParallel::Parallel(res) => {
                    let leafed = format!("{}", &res.make_statefull_tree(&mut indent.clone()));
                    printable.push_str(&leafed);
                }
            }
        }
        return printable;
    }
    fn make_stateless_tree(&self, indent: &String) -> String {
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
                        make_branch(indent.clone(), i, vec_length),
                        &res.make_statefull_tree(indent),
                    );
                    printable.push_str(&leafed);
                }

                StepOrParallel::Parallel(res) => {
                    let leafed = format!("{}", &res.make_stateless_tree(indent),);
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
    fn make_statefull_tree(&self, indent: &String) -> String {
        let printable: String;
        match self {
            StepOrParallel::Step(res) => printable = res.make_statefull_tree(indent),
            StepOrParallel::Parallel(res) => printable = res.make_statefull_tree(indent),
        }
        return printable;
    }
    fn make_stateless_tree(&self, indent: &String) -> String {
        let printable: String;
        match self {
            StepOrParallel::Step(res) => printable = res.make_stateless_tree(indent),
            StepOrParallel::Parallel(res) => printable = res.make_stateless_tree(indent),
        }
        return printable;
    }
}
impl Tree for Parallel {
    fn make_statefull_tree(&self, indent: &String) -> String {
        let mut printable: String = "".to_owned();

        // Make branch
        let vec_length = &self.steps.len() - 1;
        for (i, e) in self.steps.iter().enumerate() {
            let leafed = format!(
                "{}{}",
                make_branch(indent.clone(), i, vec_length),
                &e.make_statefull_tree(indent),
            );
            printable.push_str(&leafed);
        }
        return printable;
    }
    fn make_stateless_tree(&self, indent: &String) -> String {
        let mut printable: String = "".to_owned();
        // Make branch
        let vec_length = &self.steps.len() - 1;
        for (i, e) in self.steps.iter().enumerate() {
            let leafed = format!(
                "{}{}",
                make_branch(indent.clone(), i, vec_length),
                &e.make_statefull_tree(indent),
            );
            printable.push_str(&leafed);
        }
        return printable;
    }
}

impl Tree for Step {
    fn make_statefull_tree(&self, indent: &String) -> String {
        // let indent = INDENT.repeat(indent);
        let mut printable: String = "".to_owned();

        // Make root
        let root = format!(" step: {}\n", &self.name);
        printable.push_str(&root);

        // Make sub branch
        // indent = indent + 1;

        let vec_length = &self.commands.len() - 1;
        for (i, e) in self.commands.iter().enumerate() {
            let leafed = format!(
                "{}{}",
                make_branch(indent.clone(), i, vec_length),
                &e.make_statefull_tree(&add_level(indent)),
            );
            printable.push_str(&leafed);
        }

        return printable;
    }
    fn make_stateless_tree(&self, indent: &String) -> String {
        // let mut indent = indent;
        // let indent = INDENT.repeat(indent);
        let mut printable: String = "".to_owned();

        // Make root
        let root = format!(" step: {}\n", &self.name);
        printable.push_str(&root);

        // Make sub branch
        // indent = indent + 1;
        add_level(indent);

        let vec_length = &self.commands.len() - 1;
        for (i, e) in self.commands.iter().enumerate() {
            let leafed = format!(
                "{}{}",
                make_branch(indent.clone(), i, vec_length),
                &e.make_stateless_tree(indent),
            );
            printable.push_str(&leafed);
        }

        return printable;
    }
}
impl Tree for Command {
    fn make_statefull_tree(&self, indent: &String) -> String {
        // let mut indent = indent;
        // let mut indent = INDENT.repeat(indent);

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
                // indent = indent + 1;
                // indent = INDENT.repeat(indent);
                // let mut i = 0;
                // while i < indent {
                //     i = i + 1;
                // indent.push_str(&format!("{}", Characters::unicode().vbar));
                // indent.push_str(&INDENT);
                // }

                if self.output.clone().unwrap().stdout.is_some() {
                    let mut out = self.output.clone().unwrap().stdout.unwrap();

                    // Indent output
                    out = out.replace("\n", &format!("\n{indent:} ", indent = indent));
                    out.push_str("\n");

                    let leafed = format!("{}{}", make_branch(indent.clone(), 0, 0), out);
                    printable.push_str(&leafed);
                }
            }
            Some(Status::Failed) => {
                root = format!(" {}\n", &self.stdin.red());
                printable.push_str(&root);

                // Sub branch
                // indent = indent + 1;
                // indent = INDENT.repeat(indent);
                if self.output.clone().unwrap().stderr.is_some() {
                    let mut out = self.output.clone().unwrap().stderr.unwrap();

                    // Indent output
                    out = out.replace("\n", &format!("\n{indent:} ", indent = indent));
                    out.push_str("\n");

                    let leafed = format!("{}{}", make_branch(indent.clone(), 0, 0), out);
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
    fn make_stateless_tree(&self, indent: &String) -> String {
        let mut printable: String = "".to_owned();

        let root = format!(" {}\n", &self.stdin);
        printable.push_str(&root);

        return printable;
    }
}
