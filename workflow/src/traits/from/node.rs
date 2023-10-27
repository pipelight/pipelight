/**
Convert the pipeline object into a printable tree.
Every pipeline subcomponent such as step and commands are converted to
the node pretty printable type.
*/
// Struct
use crate::types::{Command, Event, Node, Parallel, Pipeline, Step, StepOrParallel, Trigger};
use exec::{Statuable, Status};
use log::LevelFilter;
use utils::git::Flag;
// Globals
use utils::globals::LOGGER;
// Colorize
use colored::Colorize;
// Duration
use chrono::{DateTime, Local};

impl From<&Event> for String {
    fn from(e: &Event) -> String {
        let mut string = "".to_owned();
        let mut date = e.date.parse::<DateTime<Local>>().unwrap().to_rfc2822();
        date = format!("{}\n", date);
        string.push_str(&date);

        let mut tag: Option<String> = None;
        let mut branch: Option<String> = None;
        let action: Option<Flag> = e.trigger.get_action().unwrap();
        let commit: Option<String> = e.trigger.get_commit().unwrap();
        match e.trigger.clone() {
            Trigger::TriggerTag(trigger_tag) => {
                tag = trigger_tag.tag;
            }
            Trigger::TriggerBranch(trigger_branch) => {
                branch = trigger_branch.branch;
            }
        }

        // Set the commit id
        if let Some(commit) = commit {
            let header = "commit: ";
            let commit = format!("{}{}\n", header.white(), &commit.white());
            string.push_str(&commit);
        }

        // Set the tag name
        if let Some(tag) = tag {
            let header = "tag: ";
            let tag = format!("{}{}\n", header.white(), String::from(&tag).white());
            string.push_str(&tag);
        }

        // Set the branch name
        if let Some(branch) = branch {
            let header = "branch: ";
            let branch = format!("{}{}\n", header.white(), String::from(&branch).white());
            string.push_str(&branch);
        }

        // Set the action
        if let Some(action) = action {
            let header = "action: ";
            let action = format!("{}{}\n", header.white(), String::from(&action).white());
            string.push_str(&action);
        }
        string
    }
}

impl From<&Pipeline> for Node {
    fn from(e: &Pipeline) -> Self {
        let mut head: String = "".to_owned();
        if e.status.is_some() {
            let separator = format!("{}", " - ".white());
            let status = format!("{}{}", e.status.clone().unwrap(), separator);
            head.push_str(&status);
        }
        if e.event.is_some() {
            let event = String::from(&e.event.clone().unwrap());
            head.push_str(&format!("{}", &event.white()))
        }
        head = head.to_owned();

        let name = format!("pipeline: {}", e.name.clone());
        head.push_str(&name);
        let mut children: Vec<Node> = e.steps.iter().map(Node::from).collect();

        // Duration
        // If pipeline is_running
        let mut duration: Option<String> = None;
        if e.duration.is_some() {
            duration = Some(String::from(e.duration.as_ref().unwrap()));
        }
        // Fallback
        if e.fallback.is_some() {
            if e.fallback.clone().unwrap().on_failure.is_some() {
                let on_failure = e.fallback.clone().unwrap().on_failure.unwrap();

                let on_failure_children = on_failure.iter().map(Node::from).collect();
                let node = Node {
                    children: Some(on_failure_children),
                    value: Some("on_failure".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_success.is_some() {
                let on_success = e.fallback.clone().unwrap().on_success.unwrap();
                let on_success_children = on_success.iter().map(Node::from).collect();
                let node = Node {
                    children: Some(on_success_children),
                    value: Some("on_success".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_abortion.is_some() {
                let on_abortion = e.fallback.clone().unwrap().on_abortion.unwrap();
                let on_abortion_children = on_abortion.iter().map(Node::from).collect();
                let node = Node {
                    children: Some(on_abortion_children),
                    value: Some("on_abortion".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
        }
        Node {
            value: Some(head),
            status: e.status.clone(),
            duration,
            children: Some(children),
            ..Node::default()
        }
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
        let mut children: Vec<Node> = e.steps.iter().map(Node::from).collect();

        // Duration
        let mut duration: Option<String> = None;
        if e.duration.is_some() {
            duration = Some(String::from(e.duration.as_ref().unwrap()));
        }

        // Fallback
        if e.fallback.is_some() {
            if e.fallback.clone().unwrap().on_failure.is_some() {
                let on_failure = e.fallback.clone().unwrap().on_failure.unwrap();

                let on_failure_children = on_failure.iter().map(Node::from).collect();
                let node = Node {
                    children: Some(on_failure_children),
                    value: Some("on_failure".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_success.is_some() {
                let on_success = e.fallback.clone().unwrap().on_success.unwrap();
                let on_success_children = on_success.iter().map(Node::from).collect();
                let node = Node {
                    children: Some(on_success_children),
                    value: Some("on_success".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_abortion.is_some() {
                let on_abortion = e.fallback.clone().unwrap().on_abortion.unwrap();
                let on_abortion_children = on_abortion.iter().map(Node::from).collect();
                let node = Node {
                    children: Some(on_abortion_children),
                    value: Some("on_abortion".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
        }
        Node {
            value: Some("parallel".to_owned()),
            status: e.status.clone(),
            duration,
            children: Some(children),
            level: LevelFilter::Warn,
        }
    }
}
impl From<&Step> for Node {
    fn from(e: &Step) -> Self {
        let head = format!("step: {}", e.name.clone());
        let mut children: Vec<Node> = e.commands.iter().map(Node::from).collect();

        // Duration
        let mut duration: Option<String> = None;
        if e.duration.is_some() {
            duration = Some(String::from(e.duration.as_ref().unwrap()));
        }

        // Fallback
        if e.fallback.is_some() {
            if e.fallback.clone().unwrap().on_failure.is_some() {
                let on_failure = e.fallback.clone().unwrap().on_failure.unwrap();

                let on_failure_children = on_failure.iter().map(Node::from).collect();
                let node = Node {
                    children: Some(on_failure_children),
                    value: Some("on_failure".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_success.is_some() {
                let on_success = e.fallback.clone().unwrap().on_success.unwrap();
                let on_success_children = on_success.iter().map(Node::from).collect();
                let node = Node {
                    children: Some(on_success_children),
                    value: Some("on_success".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_abortion.is_some() {
                let on_abortion = e.fallback.clone().unwrap().on_abortion.unwrap();
                let on_abortion_children = on_abortion.iter().map(Node::from).collect();
                let node = Node {
                    children: Some(on_abortion_children),
                    value: Some("on_abortion".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
        }
        Node {
            value: Some(head),
            status: e.status.clone(),
            duration,
            children: Some(children),
            level: LevelFilter::Warn,
        }
    }
}

impl From<&Command> for Node {
    fn from(e: &Command) -> Self {
        // Duration
        let mut duration: Option<String> = None;
        if e.duration.is_some() {
            duration = Some(String::from(e.duration.as_ref().unwrap()));
        }
        let mut node = Node {
            level: LevelFilter::Info,
            duration,
            ..Node::default()
        };
        // Convert command output as child node
        if e.process.io.stdout.is_some() | e.process.io.stderr.is_some() {
            let stdout = format!("stdout: {}", e.process.io.stdout.clone().unwrap());
            let stderr = format!("stderr: {}", e.process.io.stderr.clone().unwrap());

            let out = match e.get_status() {
                Some(Status::Succeeded) => e.process.io.stdout.clone(),
                Some(Status::Failed) => e.process.io.stderr.clone(),
                Some(Status::Started) => None,
                Some(Status::Aborted) => None,
                Some(Status::Running) => None,
                None => None,
            };
            let out = Node {
                value: out,
                status: e.get_status(),
                children: None,
                level: LevelFilter::Debug,
                ..Node::new()
            };
            let stdout = Node {
                value: Some(stdout),
                status: e.get_status(),
                children: None,
                level: LevelFilter::Trace,
                ..Node::new()
            };
            let stderr = Node {
                value: Some(stderr),
                status: e.get_status(),
                children: None,
                level: LevelFilter::Trace,
                ..Node::new()
            };
            if LOGGER.lock().unwrap().pipelines.level == LevelFilter::Debug {
                node.children = Some(vec![out]);
            } else {
                node.children = Some(vec![stdout, stderr]);
            }
        }
        node.value = e.process.io.stdin.clone();
        node.status = e.get_status();
        node
    }
}
