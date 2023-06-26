use crate::{Command, Event, Node, Parallel, Pipeline, Step, StepOrParallel};
use exec::{Statuable, Status};
use log::LevelFilter;

// Colorize
use chrono::{DateTime, Local};
use colored::Colorize;

impl From<&Event> for String {
    fn from(e: &Event) -> String {
        let mut string = "".to_owned();
        let mut date = e.date.parse::<DateTime<Local>>().unwrap().to_rfc2822();
        date = format!("{}\n", date);
        string.push_str(&date);

        let header = "action: ";
        let action = format!(
            "{}{}\n",
            header.white(),
            String::from(&e.trigger.action().clone().unwrap()).white()
        );
        string.push_str(&action);
        if e.trigger.tag().is_some() {
            let header = "branch: ";
            let tag = format!(
                "{}{}\n",
                header.white(),
                String::from(&e.trigger.tag().clone().unwrap()).white()
            );
            string.push_str(&tag);
        } else if e.trigger.branch().is_some() {
            let header = "branch: ";
            let branch = format!(
                "{}{}\n",
                header.white(),
                String::from(&e.trigger.branch().clone().unwrap()).white()
            );
            string.push_str(&branch);
        }
        return string;
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
        head = format!("{}", head);

        let name = format!("pipeline: {}", e.name.clone());
        head.push_str(&name);
        let mut children: Vec<Node> = e.steps.iter().map(|e| Node::from(e)).collect();

        // Fallback
        if e.fallback.is_some() {
            if e.fallback.clone().unwrap().on_failure.is_some() {
                let on_failure = e.fallback.clone().unwrap().on_failure.unwrap();

                let on_failure_children = on_failure.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_failure_children),
                    value: Some("on_failure".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_success.is_some() {
                let on_success = e.fallback.clone().unwrap().on_success.unwrap();
                let on_success_children = on_success.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_success_children),
                    value: Some("on_success".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_abortion.is_some() {
                let on_abortion = e.fallback.clone().unwrap().on_abortion.unwrap();
                let on_abortion_children = on_abortion.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_abortion_children),
                    value: Some("on_abortion".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
        }

        let node = Node {
            value: Some(head),
            status: e.status.clone(),
            duration: e.duration,
            children: Some(children),
            ..Node::default()
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
        let mut children: Vec<Node> = e.steps.iter().map(|el| Node::from(el)).collect();

        // Fallback
        if e.fallback.is_some() {
            if e.fallback.clone().unwrap().on_failure.is_some() {
                let on_failure = e.fallback.clone().unwrap().on_failure.unwrap();

                let on_failure_children = on_failure.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_failure_children),
                    value: Some("on_failure".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_success.is_some() {
                let on_success = e.fallback.clone().unwrap().on_success.unwrap();
                let on_success_children = on_success.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_success_children),
                    value: Some("on_success".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_abortion.is_some() {
                let on_abortion = e.fallback.clone().unwrap().on_abortion.unwrap();
                let on_abortion_children = on_abortion.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_abortion_children),
                    value: Some("on_abortion".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
        }

        let node = Node {
            value: Some("parallel".to_owned()),
            status: e.status.clone(),
            duration: e.duration,
            children: Some(children),
            level: LevelFilter::Warn,
            ..Node::default()
        };
        return node;
    }
}
impl From<&Step> for Node {
    fn from(e: &Step) -> Self {
        let head = format!("step: {}", e.name.clone());
        let mut children: Vec<Node> = e.commands.iter().map(|el| Node::from(el)).collect();

        // Fallback
        if e.fallback.is_some() {
            if e.fallback.clone().unwrap().on_failure.is_some() {
                let on_failure = e.fallback.clone().unwrap().on_failure.unwrap();

                let on_failure_children = on_failure.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_failure_children),
                    value: Some("on_failure".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_success.is_some() {
                let on_success = e.fallback.clone().unwrap().on_success.unwrap();
                let on_success_children = on_success.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_success_children),
                    value: Some("on_success".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_abortion.is_some() {
                let on_abortion = e.fallback.clone().unwrap().on_abortion.unwrap();
                let on_abortion_children = on_abortion.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_abortion_children),
                    value: Some("on_abortion".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
        }
        let node = Node {
            value: Some(head),
            status: e.status.clone(),
            duration: e.duration,
            children: Some(children),
            level: LevelFilter::Warn,
            ..Node::default()
        };
        return node;
    }
}

impl From<&Command> for Node {
    fn from(e: &Command) -> Self {
        let mut node = Node {
            level: LevelFilter::Info,
            duration: e.duration,
            ..Node::default()
        };
        // Convert command output as child node
        if e.process.state.stdout.is_some() | e.process.state.stderr.is_some() {
            let out = match e.get_status() {
                Some(Status::Succeeded) => e.process.state.stdout.clone(),
                Some(Status::Failed) => e.process.state.stderr.clone(),
                Some(Status::Started) => None,
                Some(Status::Aborted) => None,
                Some(Status::Running) => None,
                None => None,
            };
            let child = Node {
                value: out,
                status: e.get_status(),
                children: None,
                level: LevelFilter::Debug,
                ..Node::new()
            };
            node.children = Some(vec![child]);
        }
        node.value = e.process.state.stdin.clone();
        node.status = e.get_status();
        return node;
    }
}