use std::{cmp::min, fmt::Display};

/// A span of the input text that is in a single color
#[derive(Debug, Clone)]
pub struct ColoredSpan<'a> {
    color: Option<&'a str>,
    txt: &'a str,
}

impl<'a> ColoredSpan<'a> {
    /// Constructs a new span
    fn new(color: Option<&'a str>, txt: &'a str) -> Self {
        ColoredSpan { color, txt }
    }

    pub fn color(&self) -> Option<&'a str> {
        self.color
    }

    pub fn as_str(&self) -> &'a str {
        self.txt
    }

    /// Length of the span in chars
    pub fn char_len(&self) -> usize {
        self.txt.chars().count()
    }

    /// The position of the last space before the given index
    fn last_break_before(&self, char_idx: usize) -> Option<usize> {
        if char_idx == 0 {
            return None;
        }
        match self.txt.char_indices().nth(char_idx) {
            // we only get this if are past the end of the slice
            None => match self.txt.rmatch_indices(' ').next() {
                None => None,
                Some((idx, _)) => Some(idx),
            },
            Some((idx, _)) => match self.txt[..idx].rmatch_indices(' ').next() {
                None => None,
                Some((idx, _)) => Some(idx),
            },
        }
    }

    /// Splits the span into 2 with the index being the first char on the right side
    fn split_at_idx(&self, char_idx: usize) -> (Self, Self) {
        let idx = self
            .txt
            .char_indices()
            .nth(char_idx)
            .map(|(i, _)| i)
            .unwrap();
        (
            ColoredSpan::new(self.color, &self.txt[..idx]),
            ColoredSpan::new(self.color, &self.txt[idx..]),
        )
    }

    /// Splits the span into 2 with the index being omitted
    fn split_omitting(&self, omit_idx: usize) -> (Self, Self) {
        let idx = self
            .txt
            .char_indices()
            .nth(omit_idx)
            .map(|(i, _)| i)
            .unwrap();
        (
            ColoredSpan::new(self.color, &self.txt[..idx]),
            ColoredSpan::new(self.color, &self.txt[idx + 1..]),
        )
    }
}

// /// Show the color and text information
// impl<'a> Display for ColoredSpan<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self.color {
//             None => write!(f, "#[]{}", self.txt),
//             Some(c) => write!(f, "#[{}]{}", c, self.txt),
//         }
//     }
// }

impl<'a> ToString for ColoredSpan<'a> {
    fn to_string(&self) -> String {
        match self.color {
            None => format!("#[]{}", self.txt),
            Some(c) => format!("#[{}]{}", c, self.txt),
        }
    }
}

/// A Line of colored text, which can be made up of multiple spans
#[derive(Debug, Clone)]
pub struct ColoredLine<'a> {
    spans: Vec<ColoredSpan<'a>>,
}

impl<'a> ColoredLine<'a> {
    /// Constructs a new, empty line
    fn new() -> Self {
        ColoredLine { spans: Vec::new() }
    }

    /// Length of the line in chars
    pub fn char_len(&self) -> usize {
        self.spans.iter().fold(0, |cnt, spn| cnt + spn.char_len())
    }

    pub fn spans(&self) -> impl Iterator<Item = &ColoredSpan<'a>> {
        self.spans.iter()
    }

    /// Adds a span to the line
    fn push(&mut self, span: ColoredSpan<'a>) {
        self.spans.push(span);
    }

    /// Finds the last space in the line before the given index
    fn last_break_before(&self, char_idx: usize) -> Option<usize> {
        // println!("lbb - {}, {}", self, char_idx);
        let mut len_left = char_idx;
        let mut len_so_far = 0;
        let mut best: Option<usize> = None;

        for span in self.spans.iter() {
            if len_left == 0 {
                break;
            }
            let char_len = span.char_len();
            let my_max = min(char_len + 1, len_left);

            // println!(" - span.lbb {}, {}", span, my_max);
            match span.last_break_before(my_max) {
                None => {}
                Some(idx) => {
                    // println!(" - new best={}", len_so_far + idx);
                    best = Some(len_so_far + idx);
                }
            }
            len_left = len_left.saturating_sub(char_len);
            len_so_far += char_len;
        }

        // println!(" : result={:?}", best);
        best
    }

    /// Returns a line with the spans that make up the first word in the line
    fn first_word(&self) -> Self {
        let mut out = ColoredLine::new();
        for span in self.spans.iter() {
            match span.txt.find(" ") {
                None => out.push(span.clone()),
                Some(idx) => {
                    out.push(ColoredSpan::new(span.color, &span.txt[..idx]));
                    break;
                }
            }
        }
        out
    }

    /// Returns 2 lines where a hyphen is added to the first and the second starts at the given index
    fn hyphenate_at_char(&self, split_idx: usize) -> (Self, Self) {
        // let mut left = ColoredLine::new();
        // let mut right = ColoredLine::new();
        // let mut len_so_far = 0;

        // for span in self.spans.iter() {
        //     if len_so_far >= split_idx {
        //         right.spans.push(span.clone());
        //     } else {
        //         let char_len = span.char_len();
        //         if len_so_far + char_len == split_idx {
        //             left.spans.push(span.clone());
        //         } else if len_so_far + char_len > split_idx {
        //             let idx = split_idx - len_so_far;
        //             let (a, b) = span.split_at_idx(idx);
        //             println!("hac - sac - {} = {:?} + {:?}", idx, a, b);
        //             left.push(a);
        //             left.push(ColoredSpan::new(span.color, "-"));
        //             right.push(b);
        //         } else {
        //             left.spans.push(span.clone());
        //         }
        //         len_so_far += char_len;
        //     }
        // }

        let (mut left, right) = self.split_at_char(split_idx);
        if let Some(last_left) = left.spans.last() {
            left.push(ColoredSpan::new(last_left.color, "-"));
        };

        (left, right)
    }

    fn split_at_char(&self, split_idx: usize) -> (Self, Self) {
        let mut left = ColoredLine::new();
        let mut right = ColoredLine::new();
        let mut len_so_far = 0;

        for span in self.spans.iter() {
            if len_so_far >= split_idx {
                right.spans.push(span.clone());
            } else {
                let char_len = span.char_len();
                if len_so_far + char_len == split_idx {
                    left.spans.push(span.clone());
                } else if len_so_far + char_len > split_idx {
                    let idx = split_idx - len_so_far;
                    let (a, b) = span.split_at_idx(idx);
                    // println!("hac - sac - {} = {:?} + {:?}", idx, a, b);
                    left.push(a);
                    right.push(b);
                } else {
                    left.spans.push(span.clone());
                }
                len_so_far += char_len;
            }
        }

        (left, right)
    }

    /// Returns 2 lines where the omitted index is in neither
    fn split_omitting(&self, omit_idx: usize) -> (Self, Self) {
        //     let idx = self.0.char_indices().nth(char_idx).map(|(i,_)| i).unwrap();
        //     (Line::new(&self.0[..idx]), Line::new(&self.0[idx+1..]))
        let mut left = ColoredLine::new();
        let mut right = ColoredLine::new();
        let mut to_omit = omit_idx as i32;

        for span in self.spans.iter() {
            if to_omit < 0 {
                right.spans.push(span.clone());
            } else {
                let char_len = span.char_len() as i32;
                if to_omit < char_len {
                    let (a, b) = span.split_omitting(to_omit as usize);
                    left.push(a);
                    right.push(b);
                } else if char_len > 0 {
                    left.spans.push(span.clone());
                }
                to_omit -= char_len;
            }
        }

        (left, right)
    }
}

// /// Converts to a string that has color and text information
// impl<'a> Display for ColoredLine<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for span in self.spans.iter() {
//             match span.color {
//                 None => write!(f, "#[]{}", span.txt)?,
//                 Some(c) => write!(f, "#[{}]{}", c, span.txt)?,
//             }
//         }
//         Ok(())
//     }
// }

impl<'a> Display for ColoredLine<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = "".to_string();
        for span in self.spans.iter() {
            out.push_str(&span.to_string());
        }
        write!(f, "{}", out)
    }
}

/// Converts text into colored lines
pub fn parse_colored_lines<'a>(txt: &'a str) -> Vec<ColoredLine<'a>> {
    let mut colors: Vec<Option<&str>> = Vec::new();
    let mut out: Vec<ColoredLine<'a>> = Vec::new();

    for line in txt.split('\n') {
        let mut colored_line = parse_colored_line(line, &mut colors);
        if let Some(mut first) = colored_line.spans.get_mut(0) {
            if first.color.is_none() {
                if let Some(last_color) = colors.last() {
                    first.color = last_color.clone();
                }
            }
        }
        out.push(colored_line);
    }

    // println!("- {:?}", out);
    // println!("--");
    out
}

/// Parses a single line
pub fn parse_colored_line<'a>(line: &'a str, colors: &mut Vec<Option<&'a str>>) -> ColoredLine<'a> {
    let mut colored_line = ColoredLine::new();
    let default_color: Option<&str> = None;

    for (i, major_part) in line.split("#[").enumerate() {
        if major_part.len() == 0 {
            continue;
        } // skip empty parts
        if i == 0 {
            colored_line.push(ColoredSpan::new(default_color, major_part));
        } else if major_part.starts_with("[") {
            let c = colors.iter().last().unwrap_or(&default_color);
            colored_line.push(ColoredSpan::new(c.clone(), "#["));
            colored_line.push(ColoredSpan::new(c.clone(), &major_part[1..]));
        } else {
            match major_part.split_once("]") {
                None => panic!("Parsing error! - {}", line),
                Some((color, text)) => {
                    if color.len() == 0 {
                        colors.pop();
                    } else {
                        colors.push(Some(color));
                    }
                    let c = colors.iter().last().unwrap_or(&default_color);
                    colored_line.push(ColoredSpan::new(c.clone(), text));
                }
            }
        }
    }

    colored_line
}

/// Parses the text and wraps it into lines with a max of the given width
pub fn wrap_colored<'a>(limit: usize, text: &'a str) -> Vec<ColoredLine<'a>> {
    // println!("--------------------------------------");
    // println!("WRAP - {}: '{}'", limit, text);

    let mut output: Vec<ColoredLine<'a>> = Vec::new();

    for mut current in parse_colored_lines(text) {
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

                    let keep_len = min(limit.saturating_sub(1), first_word_len.saturating_sub(2));
                    let (left, right) = current.hyphenate_at_char(keep_len);

                    // println!("too long - {} => {} + {}", first_word, left, right);
                    // println!(": {}", left);
                    output.push(left);
                    current = right;
                }
                Some(break_index) => {
                    let (mut left, mut right) = current.split_omitting(break_index);
                    let left_len = left.char_len();
                    let line_left = limit.saturating_sub(left_len).saturating_sub(1);

                    // println!(" - left={}, line_left={}, right={}", left, line_left, right);
                    if line_left >= 4 {
                        let next_word = right.first_word();
                        let next_word_len = next_word.char_len();

                        // println!(" - : next_word={}, len={}", next_word, next_word_len);

                        if next_word_len >= 6 {
                            let keep_len = min(line_left, next_word_len - 2);
                            // println!(" - : hyphen! keep={}", keep_len);
                            (left, right) = current.hyphenate_at_char(break_index + keep_len);
                        }
                    }
                    // println!(": {}", left);
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

/// Parses the text and wraps it into lines with a max of the given width
pub fn wrap_colored_no_hyphen<'a>(limit: usize, text: &'a str) -> Vec<ColoredLine<'a>> {
    // println!("--------------------------------------");
    // println!("WRAP NO HYPHEN - {}: '{}'", limit, text);

    let mut output: Vec<ColoredLine<'a>> = Vec::new();

    for mut current in parse_colored_lines(text) {
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

                    let keep_len = min(limit, first_word_len - 1);
                    let (left, right) = current.split_at_char(keep_len);

                    // println!("too long - {} => {} + {}", first_word, left, right);
                    // println!(": {}", left);
                    if left.char_len() > 0 {
                        output.push(left);
                    }
                    current = right;
                }
                Some(break_index) => {
                    let (left, right) = current.split_omitting(break_index);
                    // let left_len = left.char_len();
                    // let line_left = limit.saturating_sub(left_len).saturating_sub(1);

                    // println!(" - left={}, line_left={}, right={}", left, line_left, right);
                    // println!(": {}", left);
                    if left.char_len() > 0 {
                        output.push(left);
                    }
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
pub fn colored_print_size(txt: &str) -> (usize, usize) {
    let mut out = (0, 0);

    for line in txt.split('\n') {
        let line_len = colored_line_len(line);
        out.0 = std::cmp::max(out.0, line_len);
        out.1 += 1;
    }

    // println!("- {:?}", out);
    // println!("--");
    out
}

/// Parses a single line
pub fn colored_line_len(line: &str) -> usize {
    let mut len = 0;
    for (i, major_part) in line.split("#[").enumerate() {
        if major_part.len() == 0 {
            continue;
        } // skip empty parts
        if i == 0 {
            len += major_part.len();
        } else if major_part.starts_with("[") {
            len += 2; // for '#['
            len += major_part.len().saturating_sub(1); // skip the leading '['
        } else {
            match major_part.split_once("]") {
                None => panic!("Parsing error! - {}", line),
                Some((_, text)) => {
                    len += text.len();
                }
            }
        }
    }

    len
}

////////////////////////////////////////////

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn span_last_break_before() {
        let text = "This is a span of text";
        let span = ColoredSpan::new(Some("color"), text);

        assert_eq!(span.last_break_before(0), None);
        assert_eq!(span.last_break_before(4), None);
        assert_eq!(span.last_break_before(5), Some(4));
        assert_eq!(span.last_break_before(12), Some(9));
        assert_eq!(span.last_break_before(20), Some(17));

        let text = "This is a ";
        let span = ColoredSpan::new(Some("color"), text);

        assert_eq!(span.last_break_before(0), None);
        assert_eq!(span.last_break_before(0), None);
        assert_eq!(span.last_break_before(4), None);
        assert_eq!(span.last_break_before(5), Some(4));
        assert_eq!(span.last_break_before(12), Some(9));
    }

    #[test]
    fn line_last_break_before() {
        let text = "This is a #[00F]span#[] of text";
        let mut colors = Vec::new();
        let line = parse_colored_line(text, &mut colors);
        // let mut buffer = Buffer::new(50, 50);

        assert_eq!(line.last_break_before(0), None);
        assert_eq!(line.last_break_before(4), None);
        assert_eq!(line.last_break_before(5), Some(4));
        assert_eq!(line.last_break_before(12), Some(9));
        assert_eq!(line.last_break_before(20), Some(17));
    }

    #[test]
    fn span_split_at_space() {
        let text = "This is a span of text";
        let span = ColoredSpan::new(Some("color"), text);

        let (left, right) = span.split_omitting(9);
        assert_eq!(left.txt, "This is a");
        assert_eq!(right.txt, "span of text");
        assert_eq!(left.color, Some("color"));
        assert_eq!(right.color, Some("color"));

        let (left, right) = span.split_omitting(0);
        assert_eq!(left.txt, "");
        assert_eq!(right.txt, "his is a span of text");
        assert_eq!(left.color, Some("color"));
        assert_eq!(right.color, Some("color"));

        let (left, right) = span.split_omitting(span.char_len() - 1);
        assert_eq!(left.txt, "This is a span of tex");
        assert_eq!(right.txt, "");
        assert_eq!(left.color, Some("color"));
        assert_eq!(right.color, Some("color"));
    }

    #[test]
    fn span_split_at_char() {
        let text = "This is a span of text";
        let span = ColoredSpan::new(Some("color"), text);

        let (left, right) = span.split_at_idx(9);
        assert_eq!(left.txt, "This is a");
        assert_eq!(right.txt, " span of text");
        assert_eq!(left.color, Some("color"));
        assert_eq!(right.color, Some("color"));

        let (left, right) = span.split_at_idx(0);
        assert_eq!(left.txt, "");
        assert_eq!(right.txt, "This is a span of text");
        assert_eq!(left.color, Some("color"));
        assert_eq!(right.color, Some("color"));

        let (left, right) = span.split_at_idx(span.char_len() - 1);
        assert_eq!(left.txt, "This is a span of tex");
        assert_eq!(right.txt, "t");
        assert_eq!(left.color, Some("color"));
        assert_eq!(right.color, Some("color"));
    }

    fn line_string(line: &ColoredLine) -> String {
        let mut out = "".to_string();
        for span in line.spans() {
            out += span.as_str();
        }
        out
    }

    #[test]
    fn line_split_at_space() {
        let text = "This is a #[00F]span#[] of text";
        let mut colors = Vec::new();
        let line = parse_colored_line(text, &mut colors);

        let (left, right) = line.split_omitting(9);
        assert_eq!(line_string(&left), "This is a");
        assert_eq!(line_string(&right), "span of text");

        let (left, right) = line.split_omitting(0);
        assert_eq!(line_string(&left), "");
        assert_eq!(line_string(&right), "his is a span of text");

        let (left, right) = line.split_omitting(line.char_len() - 1);
        assert_eq!(line_string(&left), "This is a span of tex");
        assert_eq!(line_string(&right), "");
    }

    #[test]
    fn line_hyphenate_at_char() {
        let text = "This is a #[00F]span#[] of text";
        let mut colors = Vec::new();
        let line = parse_colored_line(text, &mut colors);

        let (left, right) = line.hyphenate_at_char(12);
        assert_eq!(line_string(&left), "This is a sp-");
        assert_eq!(line_string(&right), "an of text");

        let (left, right) = line.hyphenate_at_char(0);
        assert_eq!(line_string(&left), "");
        assert_eq!(line_string(&right), "This is a span of text");

        let (left, right) = line.hyphenate_at_char(line.char_len() - 1);
        assert_eq!(line_string(&left), "This is a span of tex-");
        assert_eq!(line_string(&right), "t");
    }

    #[test]
    fn one_line() {
        let result = wrap_colored(20, "Testing");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].to_string(), "#[]Testing");
    }

    #[test]
    fn flying_period() {
        let result = wrap_colored(12, "This is a line of text.");

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].to_string(), "#[]This is a");
        assert_eq!(result[1].to_string(), "#[]line of");
        assert_eq!(result[2].to_string(), "#[]text.");
    }

    #[test]
    fn two_lines() {
        let result = wrap_colored(20, "Testing a longer bit of text.");

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].to_string(), "#[]Testing a longer bit");
        assert_eq!(result[1].to_string(), "#[]of text.");
    }

    #[test]
    fn four_lines() {
        let result = wrap_colored_no_hyphen(
            12,
            "#[red]This is some #[blue]text#[] in multiple#[] lines.",
        );

        assert_eq!(result.len(), 4);
        assert_eq!(result[0].to_string(), "#[red]This is some");
        assert_eq!(result[1].to_string(), "#[blue]text#[red] in");
        assert_eq!(result[2].to_string(), "#[red]multiple#[]");
        assert_eq!(result[3].to_string(), "#[]lines.");
    }

    #[test]
    fn three_lines() {
        let result = wrap_colored(
            20,
            "Testing a longer bit of text that spans multiple lines.",
        );

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].to_string(), "#[]Testing a longer bit");
        assert_eq!(result[1].to_string(), "#[]of text that spans");
        assert_eq!(result[2].to_string(), "#[]multiple lines.");
    }

    #[test]
    fn with_color() {
        let result = wrap_colored(
            20,
            "Testing a #[blue]longer#[] bit of text that #[red]spans multiple lines#[].",
        );

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].to_string(), "#[]Testing a #[blue]longer#[] bit");
        assert_eq!(result[1].to_string(), "#[]of text that #[red]spans");
        assert_eq!(result[2].to_string(), "#[red]multiple lines#[].");
    }

    #[test]
    fn start_color() {
        let result = wrap_colored(20, "#[color]This   is \na \ttest.");

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].to_string(), "#[color]This   is ");
        assert_eq!(result[1].to_string(), "#[color]a \ttest.");
    }
}
