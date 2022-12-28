use super::JumpAnnotation;
use crate::commands::Context;
use helix_view::{
    decorations::{TextAnnotation, TextAnnotationKind},
    graphics::{Color, Modifier, Style},
    input::KeyEvent,
    Document, Theme,
};

pub const JUMP_KEYS: &[u8] = b"etovxqpdygfblzhckisuran";

fn annotate(doc: &Document, theme: &Theme, jumps: Vec<JumpAnnotation>) -> Vec<TextAnnotation> {
    let text = doc.text().slice(..);

    let single_style = theme.try_get("ui.jump.single").unwrap_or_else(|| {
        Style::default()
            .fg(Color::Rgb(0xff, 0x00, 0x7c))
            .add_modifier(Modifier::BOLD)
    });
    let multi_first_style = theme.try_get("ui.jump.multi-first").unwrap_or_else(|| {
        Style::default()
            .fg(Color::Rgb(0x00, 0xdf, 0xff))
            .add_modifier(Modifier::BOLD)
    });
    let multi_rest_style = theme
        .try_get("ui.jump.multi-rest")
        .unwrap_or_else(|| Style::default().fg(Color::Rgb(0x2b, 0x8d, 0xb3)));

    let mut annotations: Vec<_> = jumps
        .into_iter()
        .flat_map(|jump| {
            let line = text.char_to_line(jump.loc);
            let column = jump.loc - text.line_to_char(line);
            let style = match jump.keys.len() {
                2.. => multi_first_style,
                _ => single_style,
            };
            let (first, rest) = jump.keys.split_at(1);
            let (first, rest) = (String::from(first), String::from(rest));
            let mut annotations = vec![TextAnnotation {
                text: first.into(),
                style,
                line,
                kind: TextAnnotationKind::Overlay(column),
            }];
            if !rest.is_empty() {
                annotations.push(TextAnnotation {
                    text: rest.into(),
                    style: multi_rest_style,
                    line,
                    kind: TextAnnotationKind::Overlay(column + 1),
                });
            }
            annotations.into_iter()
        })
        .collect();
    annotations.sort_by(|a, b| {
        if let (TextAnnotationKind::Overlay(col1), TextAnnotationKind::Overlay(col2)) =
            (a.kind, b.kind)
        {
            return col1.cmp(&col2);
        }
        unreachable!();
    });
    annotations
}

pub fn apply_dimming(ctx: &mut Context) {
    let (view, doc) = current!(ctx.editor);
    let first_line = view.offset.row;
    let num_lines = view.last_line(doc) - first_line + 1;

    let lines: Vec<_> = doc
        .text()
        .lines_at(first_line)
        .zip(first_line..)
        .take(num_lines)
        .map(|(line, idx)| TextAnnotation {
            text: String::from(line).into(),
            style: Style::default().fg(Color::Rgb(0x66, 0x66, 0x66)),
            line: idx,
            kind: TextAnnotationKind::Overlay(0),
        })
        .collect();
    doc.push_text_annotations("jump_mode", lines.into_iter());
}

pub fn clear_dimming(ctx: &mut Context) {
    doc_mut!(ctx.editor).clear_text_annotations("jump_mode");
}

pub fn show_key_annotations_with_callback<F>(
    ctx: &mut Context,
    annotations: Vec<JumpAnnotation>,
    on_key_press_callback: F,
) where
    F: FnOnce(&mut Context, KeyEvent) + 'static,
{
    apply_dimming(ctx);
    let doc = doc_mut!(ctx.editor);
    doc.push_text_annotations(
        "jump_mode",
        annotate(doc, &ctx.editor.theme, annotations).into_iter(),
    );
    ctx.on_next_key(on_key_press_callback);
}
