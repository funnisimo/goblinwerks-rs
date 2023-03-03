use crate::MsgData;

pub struct Messages {
    data: Option<Vec<(String, Option<MsgData>)>>,
}

impl Messages {
    pub fn new() -> Self {
        Messages {
            data: Some(Vec::new()),
        }
    }

    pub fn push(&mut self, id: &str, data: Option<MsgData>) {
        self.data.as_mut().unwrap().push((id.to_string(), data));
    }

    pub fn take(&mut self) -> Vec<(String, Option<MsgData>)> {
        self.data.replace(Vec::new()).unwrap()
    }
}
