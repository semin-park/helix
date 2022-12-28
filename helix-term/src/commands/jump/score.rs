use helix_core::{visual_coords_at_pos, Position, Range};

use crate::commands::Context;

use super::locations::cursor_at;

fn manhattan_distance(p1: &Position, p2: &Position) -> usize {
    // Make it easier to travel along the y-axis
    let x_weight = 10;
    p1.row.abs_diff(p2.row) + p1.col.abs_diff(p2.col) * x_weight
}

struct ScoredTarget {
    range: Range,
    distance: usize,
}

pub fn sort_jump_targets(cx: &mut Context, jump_targets: Vec<Range>) -> Vec<Range> {
    // Each jump target will be scored based on its distance to the cursor position.
    let cursor = cursor_at(cx);
    let doc = doc!(cx.editor);
    let text = doc.text().slice(..);
    let mut jump_targets: Vec<_> = jump_targets
        .into_iter()
        .map(|range| ScoredTarget {
            range,
            distance: manhattan_distance(
                &cursor,
                &visual_coords_at_pos(text, range.head, doc.tab_width()),
            ),
        })
        .collect();
    // Sort by the distance (shortest first)
    jump_targets.sort_by(|a, b| a.distance.cmp(&b.distance));
    jump_targets.iter().map(|a| a.range).collect()
}
