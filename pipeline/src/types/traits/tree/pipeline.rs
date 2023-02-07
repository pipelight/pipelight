// Relative paths
use super::characters::Characters;
use super::composables::{make_branch, make_statefull_tree, make_stateless_tree};
use super::Tree;

// Absolute paths
use crate::types::{Command, Event, Parallel, Pipeline, Step, StepOrParallel, Trigger};
use exec::types::{Statuable, Status};
use log::{debug, info, warn};
use std::error::Error;

static INDENT: &str = " ";

impl Tree for Pipeline {
    fn make_statefull_tree(&self, level: usize) -> String {
        let mut level = level;
        let indent = INDENT.repeat(level);
        let mut printable: String = "".to_owned();

        // Make root
        let root = format!(" pipeline: {}\n", &self.name);
        printable.push_str(&root);

        // Make sub branch
        level = level + 1;
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
        let mut level = level;
        let indent = INDENT.repeat(level);
        let mut printable: String = "".to_owned();

        // Make root
        let root = format!(" pipeline: {}\n", &self.name);
        printable.push_str(&root);

        // Make branch
        level = level + 1;
        let vec_length = &self.steps.len() - 1;
        for (i, e) in self.steps.iter().enumerate() {
            match e {
                StepOrParallel::Step(res) => {
                    let leafed = format!(
                        "{}{tag:}{}",
                        make_branch(i, vec_length, level),
                        &res.make_statefull_tree(level),
                        tag = res.name
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
