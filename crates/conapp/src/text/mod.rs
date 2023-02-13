mod plain;
pub use plain::{plain_line_len, plain_print_size, wrap_plain, wrap_plain_no_hyphen, RefLine};

mod colored;
pub use colored::{
    colored_line_len, colored_print_size, parse_colored_line, parse_colored_lines, wrap_colored,
    wrap_colored_no_hyphen, ColoredLine, ColoredSpan,
};
