use crate::types::traits::tree::characters::Characters;
use crate::types::traits::tree::Tree;
use crate::types::{Command, Event, Parallel, Pipeline, Step, StepOrParallel, Trigger};
use exec::types::{Statuable, Status};

static INDENT: &str = "  ";

impl Statuable for Command {
    fn get_status(&self) -> Option<Status> {
        return self.status.clone();
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }
}
impl Statuable for Step {
    fn get_status(&self) -> Option<Status> {
        return self.status.clone();
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }
}
impl Statuable for StepOrParallel {
    fn set_status(&mut self, status: Option<Status>) {
        match self {
            StepOrParallel::Step(res) => res.status = status,
            StepOrParallel::Parallel(res) => res.status = status,
        }
    }
    fn get_status(&self) -> Option<Status> {
        match self {
            StepOrParallel::Step(res) => res.status.clone(),
            StepOrParallel::Parallel(res) => res.status.clone(),
        }
    }
}
impl Statuable for Parallel {
    fn get_status(&self) -> Option<Status> {
        return self.status.clone();
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }
}
impl Statuable for Pipeline {
    fn get_status(&self) -> Option<Status> {
        return self.status.clone();
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }
}
pub fn make_branch(index: usize, vec_length: usize, level: usize) -> String {
    let indent = INDENT.repeat(level);
    let leaf: String;
    if index < vec_length {
        leaf = format!(
            "{indent:}{}\n{indent:}{}{}",
            Characters::unicode().vbar,
            Characters::unicode().lcross,
            Characters::unicode().hbar,
            indent = indent,
        );
    } else {
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

pub fn make_statefull_tree<T: Statuable + Tree>(vec: Vec<T>, level: usize) -> String {
    // Init variables
    let mut level = level;
    let indent = INDENT.repeat(level);
    let mut printable: String = "".to_owned();

    // Sub branches
    level = level + 1;
    let vec_length = vec.len() - 1;

    for (i, e) in vec.iter().enumerate() {
        let e_with_leaf = format!(
            "{}{}",
            make_branch(i, vec_length, level),
            e.make_statefull_tree(level)
        );
        printable.push_str(&e_with_leaf);

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
pub fn make_stateless_tree<T: Tree>(vec: Vec<T>, level: usize) -> String {
    // Init variables
    let mut level = level;
    let indent = INDENT.repeat(level);
    let mut printable: String = "".to_owned();

    // Sub branches
    level = level + 1;
    let vec_length = vec.len() - 1;
    for (i, e) in vec.iter().enumerate() {
        let e_with_leaf = format!(
            "{}{}",
            make_branch(i, vec_length, level),
            e.make_stateless_tree(level)
        );
        printable.push_str(&e_with_leaf);
    }
    return printable;
}
