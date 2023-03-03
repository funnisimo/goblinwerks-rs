use std::cmp::min;

pub struct Line(String, bool);

impl Line {
    pub fn new(source: &RefLine) -> Self {
        Line(source.as_str().to_owned(), source.has_hyphen())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn char_len(&self) -> usize {
        self.0.chars().count() + if self.1 { 1 } else { 0 }
    }

    pub fn has_hyphen(&self) -> bool {
        self.1
    }
}

pub struct RefLine<'a>(&'a str, bool);

impl<'a> RefLine<'a> {
    pub fn new(t: &'a str) -> Self {
        RefLine(t, false)
    }

    pub fn as_str(&self) -> &'a str {
        self.0
    }

    pub fn char_len(&self) -> usize {
        self.0.chars().count() + if self.1 { 1 } else { 0 }
    }

    pub fn has_hyphen(&self) -> bool {
        self.1
    }

    pub fn to_line(&self) -> Line {
        Line::new(self)
    }

    // pub fn len(&self) -> usize {
    //     self.0.chars().count() + if self.1 { 1 } else { 0 }
    // }

    fn with_hyphen(mut self) -> Self {
        self.1 = true;
        self
    }

    fn last_break_before(&self, char_idx: usize) -> Option<usize> {
        let idx = self.0.char_indices().nth(char_idx).map(|(i, _)| i).unwrap();
        match self.0[..idx].rmatch_indices(' ').next() {
            None => None,
            Some((idx, _)) => Some(idx),
        }
    }

    fn first_word(&self) -> Self {
        match self.0.find(" ") {
            None => RefLine::new(self.0),
            Some(idx) => RefLine::new(&self.0[..idx]),
        }
    }

    // pub fn left(&self, len: usize) -> Self {
    //     Line::new(&self.0[..len])
    // }

    fn hyphenate_at_char(&self, char_idx: usize) -> (Self, Self) {
        let idx = self.0.char_indices().nth(char_idx).map(|(i, _)| i).unwrap();
        (
            RefLine::new(&self.0[..idx]).with_hyphen(),
            RefLine::new(&self.0[idx..]),
        )
    }

    fn split_at_char(&self, char_idx: usize) -> (Self, Self) {
        let idx = self.0.char_indices().nth(char_idx).map(|(i, _)| i).unwrap();
        (RefLine::new(&self.0[..idx]), RefLine::new(&self.0[idx..]))
    }

    fn split_at_space(&self, char_idx: usize) -> (Self, Self) {
        let idx = self.0.char_indices().nth(char_idx).map(|(i, _)| i).unwrap();
        (
            RefLine::new(&self.0[..idx]),
            RefLine::new(&self.0[idx + 1..]),
        )
    }
}

impl<'a> ToString for RefLine<'a> {
    fn to_string(&self) -> String {
        match self.has_hyphen() {
            false => self.0.to_owned(),
            true => format!("{}-", self.0),
        }
    }
}

pub fn wrap_plain<'a>(limit: usize, text: &'a str) -> Vec<RefLine<'a>> {
    // println!("--------------------------------------");
    // println!("WRAP - {}: '{}'", limit, text);

    let mut output: Vec<RefLine<'a>> = Vec::new();

    for line in text.split('\n') {
        let mut current = RefLine::new(line);
        let mut i = 0;

        while current.char_len() > limit {
            i += 1;
            if i > 10 {
                break;
            }

            match current.last_break_before(limit + 1) {
                None => {
                    let first_word = current.first_word();
                    let first_word_len = first_word.char_len();

                    // println!("too long - {}", first_word.0);

                    let keep_len = min(limit - 1, first_word_len - 2);
                    let (left, right) = current.hyphenate_at_char(keep_len);
                    output.push(left);
                    current = right;
                }
                Some(break_index) => {
                    let (mut left, mut right) = current.split_at_space(break_index);
                    let left_len = left.char_len();
                    let line_left = limit.saturating_sub(left_len).saturating_sub(1);

                    // println!(
                    //     " - left={}, line_left={}, right={}",
                    //     left.0, line_left, right.0
                    // );
                    if line_left >= 4 {
                        let next_word = right.first_word();
                        let next_word_len = next_word.char_len();

                        // println!(" - : next_word={}, len={}", next_word.0, next_word_len);

                        if next_word_len >= 6 {
                            let keep_len = min(line_left, next_word_len - 2);
                            // println!(" - : hyphen! keep={}", keep_len);
                            (left, right) = current.hyphenate_at_char(break_index + keep_len);
                        }
                    }
                    output.push(left);
                    current = right;
                }
            }
        }

        if current.char_len() > 0 {
            output.push(current);
        }
    }
    output
}

pub fn wrap_plain_no_hyphen<'a>(limit: usize, text: &'a str) -> Vec<RefLine<'a>> {
    // println!("--------------------------------------");
    // println!("WRAP - {}: '{}'", limit, text);

    let mut output: Vec<RefLine<'a>> = Vec::new();

    for line in text.split('\n') {
        let mut current = RefLine::new(line);
        let mut i = 0;

        while current.char_len() > limit {
            i += 1;
            if i > 10 {
                break;
            }

            match current.last_break_before(limit + 1) {
                None => {
                    let first_word = current.first_word();
                    let first_word_len = first_word.char_len();

                    println!("too long - {}", first_word.0);

                    let keep_len = min(limit, first_word_len - 1);
                    let (left, right) = current.split_at_char(keep_len);
                    output.push(left);
                    current = right;
                }
                Some(break_index) => {
                    let (left, right) = current.split_at_space(break_index);
                    let left_len = left.char_len();
                    let line_left = limit.saturating_sub(left_len).saturating_sub(1);

                    println!(
                        " - left={}, line_left={}, right={}",
                        left.0, line_left, right.0
                    );
                    // if line_left >= 4 {
                    //     let next_word = right.first_word();
                    //     let next_word_len = next_word.char_len();

                    //     println!(" - : next_word={}, len={}", next_word.0, next_word_len);

                    //     if next_word_len >= 6 {
                    //         let keep_len = min(line_left, next_word_len - 2);
                    //         println!(" - : hyphen! keep={}", keep_len);
                    //         (left, right) = current.hyphenate_at_char(break_index + keep_len);
                    //     }
                    // }
                    output.push(left);
                    current = right;
                }
            }
        }

        if current.char_len() > 0 {
            output.push(current);
        }
    }
    output
}

////////////////////////////////////////////

/// Converts text into colored lines
pub fn plain_print_size(txt: &str) -> (usize, usize) {
    let mut out = (0, 0);

    for line in txt.split('\n') {
        let line_len = plain_line_len(line);
        out.0 = std::cmp::max(out.0, line_len);
        out.1 += 1;
    }

    // println!("- {:?}", out);
    // println!("--");
    out
}

/// Parses a single line
pub fn plain_line_len(line: &str) -> usize {
    line.len()
}

////////////////////////////////////////////
