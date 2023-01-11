use super::locations::cursor_at;
use crate::commands::Context;
use helix_core::{Position, Range};

fn manhattan_distance(p1: &Position, p2: &Position) -> usize {
    // Make it easier to travel along the x-axis
    const Y_WEIGHT: usize = 10;
    Y_WEIGHT
        .saturating_mul(p1.row.abs_diff(p2.row))
        .saturating_add(p1.col.abs_diff(p2.col))
}

struct ScoredTarget {
    range: Range,
    distance: usize,
}

pub fn sort_jump_targets(cx: &mut Context, jump_targets: Vec<Range>) -> Vec<Range> {
    // Each jump target will be scored based on its distance to the cursor position.
    let cursor = cursor_at(cx);
    let (view, doc) = current!(cx.editor);
    let text = doc.text().slice(..);
    let mut jump_targets: Vec<_> = jump_targets
        .into_iter()
        .map(|range| ScoredTarget {
            range,
            distance: manhattan_distance(
                &cursor,
                &view
                    .screen_coords_at_pos(doc, text, range.head)
                    .unwrap_or(Position {
                        row: usize::max_value(),
                        col: 0,
                    }),
            ),
        })
        .collect();
    // Sort by the distance (shortest first)
    jump_targets.sort_by(|a, b| a.distance.cmp(&b.distance));
    jump_targets.iter().map(|a| a.range).collect()
}
