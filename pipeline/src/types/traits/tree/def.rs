use crate::types::traits::tree::characters::Characters;
use crate::types::{Command, Config, Event, Parallel, Pipeline, Step, StepOrParallel, Trigger};
use exec::types::{Status, StrOutput};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

static INDENT: &str = "  ";

#[derive(Debug, Clone, PartialEq)]
pub struct DrawParams {
    depth: usize,
    level: usize,
    prefix: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    value: Option<String>,
    status: Option<Status>,
    children: Option<Vec<Node>>,
    params: DrawParams,
}
impl Default for Node {
    fn default() -> Self {
        Node {
            value: None,
            status: None,
            children: None,
            params: DrawParams {
                depth: 0,
                level: 0,
                prefix: None,
            },
        }
    }
}
impl Node {
    pub fn new() -> Node {
        Self::default()
    }
}

impl From<&Pipeline> for Node {
    fn from(e: &Pipeline) -> Self {
        let tag = format!("pipeline: {}", e.name.clone());
        let mut node = Node {
            value: Some(tag),
            status: e.status.clone(),
            children: None,
            ..Node::default()
        };
        let children = e
            .steps
            .iter()
            .map(|el| {
                let mut child_node = Node::from(el);
                return child_node;
            })
            .collect();
        node = Node {
            children: Some(children),
            ..node
        };

        return node;
    }
}
impl From<&StepOrParallel> for Node {
    fn from(e: &StepOrParallel) -> Self {
        match e {
            StepOrParallel::Step(res) => Node::from(res),
            StepOrParallel::Parallel(res) => Node::from(res),
        }
    }
}
impl From<&Parallel> for Node {
    fn from(e: &Parallel) -> Self {
        let mut node = Node::new();
        let children = e.steps.iter().map(|el| Node::from(el)).collect();
        node.children = Some(children);
        return node;
    }
}
impl From<&Step> for Node {
    fn from(e: &Step) -> Self {
        let tag = format!("step: {}", e.name.clone());
        let children = e.commands.iter().map(|el| Node::from(el)).collect();
        let node = Node {
            value: Some(tag),
            status: e.status.clone(),
            children: Some(children),
            ..Node::new()
        };
        return node;
    }
}

impl From<&Command> for Node {
    fn from(e: &Command) -> Self {
        let mut node = Node::default();
        // Command Output as child
        let mut children = None;
        if e.output.is_some() {
            let out = match e.status {
                Some(Status::Succeeded) => e.output.clone().unwrap().stdout,
                Some(Status::Failed) => e.output.clone().unwrap().stderr,
                Some(Status::Started) => None,
                Some(Status::Aborted) => None,
                Some(Status::Running) => None,
                None => None,
            };
            let child = Node {
                value: out,
                status: e.clone().status,
                children: None,
                ..Node::new()
            };
            children = Some(vec![child]);
        }
        node.value = Some(e.stdin.clone());
        node.status = e.status.clone();
        node.children = children;
        return node;
    }
}

impl Node {
    /// Add a leaf to prefix T from inside [T] of nth element
    pub fn leaf(&self, prefix: String, index: usize, length: usize) -> String {
        let leaf: String;
        if index == length {
            leaf = format!(
                "{prefix:}{}\n{prefix:}{}{}",
                Characters::unicode().vbar,
                Characters::unicode().lbot,
                Characters::unicode().hbar,
            );
        } else {
            leaf = format!(
                "{prefix:}{}\n{prefix:}{}{}",
                Characters::unicode().vbar,
                Characters::unicode().lcross,
                Characters::unicode().hbar,
            );
        }
        return leaf;
    }
}

fn add_level(prefix: String) -> String {
    let leaf: String = format!("{}{INDENT:}", Characters::unicode().vbar, INDENT = INDENT);
    let mut prefix = prefix;
    prefix.push_str(&leaf);
    return prefix.to_owned();
}
fn add_level_phantom(prefix: String) -> String {
    let leaf: String = format!("{}{INDENT:}", " ".to_owned());
    let mut prefix = prefix;
    prefix.push_str(&leaf);
    return prefix.to_owned();
}

impl Node {
    fn display(&self, prefix: String) {
        // Display node value
        if self.value.is_some() {
            let value = self
                .value
                .clone()
                .unwrap()
                .replace("\n", &format!("\n{prefix:} ", prefix = prefix));
            error!(target: "nude", "{}\n", value);
        }

        if self.children.is_some() {
            let length = self.children.clone().unwrap().len() - 1;

            for (index, child) in &mut self.children.clone().unwrap().iter().enumerate() {
                error!(target: "nude","{}", child.leaf(prefix.clone(), index, length));
                if index == length {
                    let prefix = add_level_phantom(prefix.clone());
                    child.display(prefix.clone());
                } else {
                    let prefix = add_level(prefix.clone());
                    child.display(prefix.clone());
                }
                // Add branching level
                // let mut prefix = prefix.clone();

                // child.add_leaf(prefix).display(prefix);
                // let mut binding = old_prefix.clone();
                // prefix = binding;
                // let mut binding = old_prefix.clone();
            }
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = "".to_owned();
        self.clone().display(prefix);
        Ok(())
    }
}
