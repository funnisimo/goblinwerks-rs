use std::collections::VecDeque;

use gw_app::log;

pub struct MessageInfo {
    pub msg: String,
    pub acked: bool,
}

impl MessageInfo {
    pub fn new(msg: String) -> MessageInfo {
        MessageInfo { msg, acked: false }
    }
}

// TODO - Location recording/filtering

pub struct Logger {
    msgs: VecDeque<MessageInfo>,
    count: usize,
    pub debug: bool,
    combat: Option<String>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            msgs: VecDeque::new(),
            debug: false,
            count: 10,
            combat: None,
        }
    }

    pub fn iter(&mut self) -> impl Iterator<Item = &MessageInfo> {
        if self.combat.is_some() {
            let combat = self.combat.take().unwrap();
            self.log(combat);
        }
        self.msgs.iter()
    }

    pub fn set_max_len(&mut self, count: usize) {
        self.count = count;
    }

    pub fn len(&self) -> usize {
        self.msgs.len()
    }

    fn trim(&mut self) {
        while self.msgs.len() > self.count {
            self.msgs.pop_back();
        }
    }

    pub fn clear(&mut self) {
        self.msgs.clear();
    }

    pub fn log<S: ToString>(&mut self, msg: S) {
        if let Some(combat) = self.combat.take() {
            self.log(combat);
        }

        let msg = msg.to_string();
        println!(":: {}", msg);
        self.msgs.push_front(MessageInfo::new(msg));
        self.trim();
    }

    pub fn log_combat<S: ToString>(&mut self, msg: S, with_comma: bool) {
        if self.combat.is_none() {
            self.combat = Some(msg.to_string());
        } else {
            let mut combat = self.combat.take().unwrap();
            combat += match with_comma {
                true => ", ",
                false => " ",
            };
            combat.push_str(&msg.to_string());
            self.combat = Some(combat);
        }
    }

    pub fn debug<S: ToString>(&mut self, msg: S) {
        if self.debug {
            self.log(msg);
        } else {
            log(&msg.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::prelude::*;

    #[test]
    fn add_msg() {
        let mut logger = Logger::new();

        logger.clear();
        logger.log("testing");
        logger.log("testing");
        logger.log("testing");

        assert_eq!(logger.iter().count(), 3);
    }

    #[test]
    fn all_trim_to() {
        let mut logger = Logger::new();

        logger.log("testing 1");
        logger.log("testing 2");
        logger.log("testing 3".to_owned());
        logger.log("testing 4");
        logger.log(format!("testing 5"));
        logger.log("testing 6");
        assert_eq!(logger.iter().count(), 6);
        assert_eq!(logger.iter().last().unwrap().msg, "testing 1");
        logger.set_max_len(4);
        assert_eq!(logger.iter().count(), 4);
        assert_eq!(logger.iter().last().unwrap().msg, "testing 3");
    }
}
