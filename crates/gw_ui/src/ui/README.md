## UI

#### Styles

- margin
- padding
- style
- computed style
- stylesheet

#### Positioning

- How to position the children once we figure out the position of the dialog/panel/body?
- How to position button sets where buttons hug the L&R edges of their parent?
  - SPAN(align=MAX)[BTN,BTN] :: A horizontal span where 2 buttons are next to each other and pushed to the right of the parent?

```rust
let my_pos = (10,5);
let my_size = (14,5);
let x = my_pos.0;
let mut y = my_pos.1;
for child in self.children().iter_mut() {
    child.set_position(x, y, my_pos, my_size);
    y += child.size.1;
}
```

- Need to build nodes first
  - with pad/margin
- Then compute sizes
- Then position parent
- Then position children

#### anchor, vanchor

- on dialog, panel
- drives pos calcuation (if pos not set)
- body is always in "whats left" area
