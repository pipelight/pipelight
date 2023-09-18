// Convert the pipeline object into a prinatbale tree.
// Every pipeline subcomponent such as step and commands are converted to
// the node pretty printable type.

use crate::globals::LOGGER;
use crate::workflow::types::{Command, Event, Node, Parallel, Pipeline, Step, StepOrParallel};
use exec::{Statuable, Status};
use log::LevelFilter;

// Colorize
use colored::Colorize;

// Duration
use crate::workflow::methods::{compute_duration, std_duration_to_iso8601};
use chrono::{DateTime, Local};

impl From<&Event> for String {
    fn from(e: &Event) -> String {
        let mut string = "".to_owned();
        let mut date = e.date.parse::<DateTime<Local>>().unwrap().to_rfc2822();
        date = format!("{}\n", date);
        string.push_str(&date);
        if e.commit.is_some() {
            let header = "commit: ";
            let commit = format!("{}{}\n", header.white(), e.commit.clone().unwrap().white());
            string.push_str(&commit);
        }
        if e.trigger.tag.is_some() {
            let header = "tag: ";
            let tag = format!(
                "{}{}\n",
                header.white(),
                String::from(&e.trigger.clone().tag.unwrap()).white()
            );
            string.push_str(&tag);
        } else if e.trigger.branch.is_some() {
            let header = "branch: ";
            let branch = format!(
                "{}{}\n",
                header.white(),
                String::from(&e.trigger.clone().branch.unwrap()).white()
            );
            string.push_str(&branch);
        }
        let header = "action: ";
        let action = format!(
            "{}{}\n",
            header.white(),
            String::from(&e.trigger.clone().action.unwrap()).white()
        );
        string.push_str(&action);
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
        let mut computed_duration: Option<String> = None;
        if e.duration.is_some() {
            if e.duration.clone().unwrap().computed.is_none() {
                computed_duration =
                    std_duration_to_iso8601(compute_duration(e.duration.clone().unwrap()).unwrap())
                        .ok();
            } else {
                computed_duration = e.duration.clone().unwrap().computed;
            }
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
            duration: computed_duration,
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
        let mut computed_duration: Option<String> = None;
        if e.duration.is_some() {
            if e.duration.clone().unwrap().computed.is_none() {
                computed_duration =
                    std_duration_to_iso8601(compute_duration(e.duration.clone().unwrap()).unwrap())
                        .ok();
            } else {
                computed_duration = e.duration.clone().unwrap().computed;
            }
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
            duration: computed_duration,
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
        let mut computed_duration: Option<String> = None;
        if e.duration.is_some() {
            if e.duration.clone().unwrap().computed.is_none() {
                computed_duration =
                    std_duration_to_iso8601(compute_duration(e.duration.clone().unwrap()).unwrap())
                        .ok();
            } else {
                computed_duration = e.duration.clone().unwrap().computed;
            }
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
            duration: computed_duration,
            children: Some(children),
            level: LevelFilter::Warn,
        }
    }
}

impl From<&Command> for Node {
    fn from(e: &Command) -> Self {
        // Duration
        let mut computed_duration: Option<String> = None;
        if e.duration.is_some() {
            if e.duration.clone().unwrap().computed.is_none() {
                computed_duration =
                    std_duration_to_iso8601(compute_duration(e.duration.clone().unwrap()).unwrap())
                        .ok();
            } else {
                computed_duration = e.duration.clone().unwrap().computed;
            }
        }
        let mut node = Node {
            level: LevelFilter::Info,
            duration: computed_duration,
            ..Node::default()
        };
        // Convert command output as child node
        if e.process.state.stdout.is_some() | e.process.state.stderr.is_some() {
            let stdout = format!("stdout: {}", e.process.state.stdout.clone().unwrap());
            let stderr = format!("stderr: {}", e.process.state.stderr.clone().unwrap());

            let out = match e.get_status() {
                Some(Status::Succeeded) => e.process.state.stdout.clone(),
                Some(Status::Failed) => e.process.state.stderr.clone(),
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
        node.value = e.process.state.stdin.clone();
        node.status = e.get_status();
        node
    }
}
