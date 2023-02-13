use super::*;
use conapp::color::get_color_opt;
use conapp::color::RGBA;
use conapp::console;
use conapp::AppContext;
use conapp::BorderType;
use conapp::LoadError;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    pub static ref STYLES: Mutex<StyleSheet> = Mutex::new(StyleSheet::new());
}

fn to_color(text: &str) -> Option<RGBA> {
    match text.parse() {
        Err(_) => None,
        Ok(rgba) => Some(rgba),
    }
}

pub fn setup_stylesheet<F>(func: F)
where
    F: FnOnce(&mut StyleSheet) -> (),
{
    let mut sheet = STYLES.lock().unwrap();
    func(&mut *sheet);
}

#[derive(Debug)]
pub struct StyleSheet {
    styles: Vec<Arc<Style>>,
}

impl StyleSheet {
    pub fn new() -> Self {
        StyleSheet { styles: Vec::new() }
    }

    pub fn insert(&mut self, style: Style) {
        self.styles.push(Arc::new(style));
    }

    pub fn load_file(&mut self, filename: &str) -> Result<(), String> {
        match read_to_string(filename) {
            Err(_) => Err(format!("Failed to open file - {}", filename)),
            Ok(file) => {
                println!("Loading stylesheet - {}", filename);
                self.load_string(file)
            }
        }
    }

    pub fn load_string(&mut self, data: String) -> Result<(), String> {
        let no_comments = strip_comments(&data);

        match make_style_sets(&no_comments) {
            Err(e) => return Err(e),
            Ok(data) => {
                for item in data.iter() {
                    match (*item.0).parse() {
                        Err(e) => return Err(e),
                        Ok(selector) => {
                            let mut style = Style::new(selector);
                            for (key, value) in item.1.iter() {
                                if *key == "fg" || *key == "color" {
                                    match get_color_opt(*value) {
                                        None => {
                                            console(format!("Failed to convert color: {}", value))
                                        }
                                        Some(rgb) => style.set_fg(rgb),
                                    }
                                } else if *key == "bg" || *key == "background-color" {
                                    match get_color_opt(*value) {
                                        None => {
                                            console(format!("Failed to convert color: {}", value))
                                        }
                                        Some(rgb) => style.set_bg(rgb),
                                    }
                                } else if *key == "border-color" {
                                    match get_color_opt(*value) {
                                        None => {
                                            console(format!("Failed to convert color: {}", value))
                                        }
                                        Some(rgb) => style.set_border_fg(rgb),
                                    }
                                } else if *key == "border-block-color" {
                                    match get_color_opt(*value) {
                                        None => {
                                            console(format!("Failed to convert color: {}", value))
                                        }
                                        Some(rgb) => style.set_border_bg(rgb),
                                    }
                                } else if *key == "border" {
                                    match value.parse::<u32>() {
                                        Err(_) => {}
                                        Ok(v) => style.set_border(match v {
                                            0 => None,
                                            1 => Some(BorderType::Single),
                                            2 => Some(BorderType::Double),
                                            _ => Some(BorderType::Color),
                                        }),
                                    }
                                } else if *key == "accent-color" {
                                    match get_color_opt(*value) {
                                        None => {
                                            console(format!("Failed to convert color: {}", value))
                                        }
                                        Some(rgb) => style.set_accent_fg(rgb),
                                    }
                                }
                            }
                            self.insert(style);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn finish(&mut self) {
        self.styles
            .sort_by(|a, b| a.score().partial_cmp(&b.score()).unwrap());
    }

    fn get_all_for_element(&self, el: &Element) -> Vec<Arc<Style>> {
        self.styles
            .iter()
            .filter_map(|s| {
                if s.is_base_match(el) {
                    return Some(s.clone());
                }
                None
            })
            .collect()
    }

    pub fn get_computed_style(&self, el: &Element) -> ComputedStyle {
        let styles = self.get_all_for_element(el);
        ComputedStyle::new(styles, &el)
    }

    pub fn dump(&self) {
        console("STYLESHEET");
        for style in self.styles.iter() {
            console(format!("{:?}", &style));
        }
    }
}

fn strip_comments(text: &str) -> String {
    let mut output = "".to_owned();

    let mut text = text;
    let mut pos = text.find("/*");
    loop {
        match pos {
            None => break,
            Some(idx) => {
                output += &text[..idx];
                text = &text[idx..];
                match text.find("*/") {
                    None => {
                        text = &"";
                        break;
                    }
                    Some(idx) => {
                        text = &text[idx + 2..];
                    }
                }
                pos = text.find("/*");
            }
        }
    }
    output += text;
    output
}

fn make_style_sets<'a>(text: &'a str) -> Result<HashMap<&'a str, Vec<(&'a str, &'a str)>>, String> {
    let mut output = HashMap::new();

    let mut text = text.trim();
    let mut idx: Option<usize> = text.find("{");
    loop {
        match idx {
            None => break,
            Some(loc) => {
                let selector = text[..loc].trim();
                text = &text[loc + 1..];
                match text.find("}") {
                    None => break,
                    Some(loc) => {
                        let body = &text[..loc];
                        if body.contains("{") {
                            return Err(format!(
                                "Parsing error @ selector = {} - found unexpected '{{'",
                                selector
                            ));
                        }
                        text = &text[loc + 1..];
                        let parts: Vec<(&str, &str)> = body
                            .split(";")
                            .map(|p| p.trim())
                            .filter(|p| p.len() > 0)
                            .filter_map(|p| {
                                let key_val: Vec<&str> = p.split(":").map(|p| p.trim()).collect();
                                if key_val.len() != 2 {
                                    return None;
                                }
                                Some((*key_val.get(0).unwrap(), *key_val.get(1).unwrap()))
                            })
                            .collect();

                        output.insert(selector, parts);
                    }
                }
                idx = text.find("{");
            }
        }
    }

    if text.len() > 0 {
        return Err(format!("File end is not correct - {}", text));
    }

    Ok(output)
}

/// Utility function to help with processing css files loaded by the runner/app context.
pub fn load_stylesheet_data(data: Vec<u8>, _: &mut AppContext) -> Result<(), LoadError> {
    match String::from_utf8(data) {
        Err(_) => {
            panic!("Failed to load css file.");
        }
        Ok(string) => {
            setup_stylesheet(|sheet| match sheet.load_string(string) {
                _ => {}
            });
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn strip_comments_easy() {
        let text = r"
        /* comment */ 
        selector { /* comment */
            key:value;
            key: /* comment */ value;
        }
        ";

        let no_comments = strip_comments(text);
        assert_eq!(
            no_comments.trim(),
            "selector { 
            key:value;
            key:  value;
        }"
        );
    }
}
