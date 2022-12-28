use crate::commands::Context;
use helix_core::{chars::char_is_word, graphemes, movement, visual_coords_at_pos, Position, Range};

fn view_boundary(cx: &Context) -> (usize, usize) {
    let (view, doc) = current_ref!(cx.editor);
    let text = doc.text().slice(..);

    let start_idx = text.line_to_char(view.offset.row);
    let end_idx = text.line_to_char(view.last_line(doc) + 1);
    (start_idx, end_idx)
}

pub fn cursor_at(cx: &Context) -> Position {
    let (view, doc) = current_ref!(cx.editor);
    let text = doc.text().slice(..);
    let cursor_at = doc.selection(view.id).primary().head;
    visual_coords_at_pos(text, cursor_at, doc.tab_width())
}

pub fn find_all_identifiers_in_view(cx: &mut Context) -> Vec<Range> {
    let (start_idx, end_idx) = view_boundary(cx);

    let (view, doc) = current!(cx.editor);
    let text = doc.text().slice(..);

    let mut jump_targets: Vec<Range> = Vec::new();
    let mut next = Range::new(start_idx, start_idx);

    // If the first line in view has a single character with no trailing whitespace,
    // `move_next_word_start` will skip it. Thus we need to handle this edge case here.
    if graphemes::is_grapheme_boundary(text, start_idx) {
        // If there is an alphanumeric character on start_idx, consider it as a target.
        let c = text.chars_at(start_idx).next().unwrap_or(' ');
        if char_is_word(c) {
            jump_targets.push(Range::point(start_idx));
        }
    }
    // Find other identifiers within this view.
    loop {
        next = movement::move_next_word_start(text, next, 1);
        // next.anchor points to the start of the identifier, and next.head
        // points to the end of the identifier. We want the cursor to be at
        // the start of the identifier, so swap the head and anchor.
        let (head, anchor) = (next.anchor, next.head);
        if anchor >= end_idx {
            break;
        }
        let c = text.chars_at(head).next().unwrap();
        if !char_is_word(c) || !view.is_cursor_in_view(head, doc, 0) {
            continue;
        }
        jump_targets.push(Range::new(anchor, head));
    }
    jump_targets
}

pub fn find_all_char_occurrences(cx: &Context, key: u8) -> Vec<Range> {
    let (start_idx, end_idx) = view_boundary(cx);
    let doc = doc!(cx.editor);
    let text = doc.text().slice(..);

    (start_idx..end_idx)
        .filter(|&idx| key == text.char(idx) as u8)
        .map(Range::point)
        .collect()
}
