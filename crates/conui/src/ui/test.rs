use conapp::Buffer;

pub fn extract_line(buf: &Buffer, x: i32, y: i32, width: i32) -> String {
    let mut output = "".to_string();
    for cx in x..x + width {
        if let Some(g) = buf.get_glyph(cx, y) {
            output.push(char::from_u32(*g).unwrap());
        }
    }
    output
}
