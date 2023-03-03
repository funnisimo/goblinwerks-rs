use gw_util::value::Value;

pub struct Messages {
    data: Option<Vec<(String, Option<Value>)>>,
}

impl Messages {
    pub fn new() -> Self {
        Messages {
            data: Some(Vec::new()),
        }
    }

    pub fn push(&mut self, id: &str, data: Option<Value>) {
        self.data.as_mut().unwrap().push((id.to_string(), data));
    }

    pub fn take(&mut self) -> Vec<(String, Option<Value>)> {
        self.data.replace(Vec::new()).unwrap()
    }
}
