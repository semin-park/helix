use std::{cell::Cell, collections::HashMap, rc::Rc};

use super::JumpAnnotation;
use crate::commands::Context;
use helix_core::text_annotations::{Overlay, TextAnnotations};
use helix_view::{
    graphics::{Color, Modifier, Style},
    input::KeyEvent,
    Document, Theme,
};

pub const JUMP_KEYS: &[u8] = b"etovxqpdygfblzhckisuran";

pub fn apply_dimming(_ctx: &mut Context) {
    // let (view, doc) = current!(ctx.editor);
    // let first_line = view.offset.row;
    // let num_lines = view.last_line(doc) - first_line + 1;

    // let lines: Vec<_> = doc
    //     .text()
    //     .lines_at(first_line)
    //     .zip(first_line..)
    //     .take(num_lines)
    //     .map(|(line, idx)| TextAnnotation {
    //         text: String::from(line).into(),
    //         style: Style::default().fg(Color::Rgb(0x66, 0x66, 0x66)),
    //         line: idx,
    //         kind: TextAnnotationKind::Overlay(0),
    //     })
    //     .collect();
    // doc.push_text_annotations("jump_mode", lines.into_iter());
}

pub fn clear_dimming(ctx: &mut Context) {
    // doc_mut!(ctx.editor).clear_text_annotations("jump_mode");
    let mut view = view_mut!(ctx.editor);
    view.visual_jump_labels[0] = Rc::new([]);
    view.visual_jump_labels[1] = Rc::new([]);
    view.visual_jump_labels[2] = Rc::new([]);
}

fn apply_style() {
    // TODO: apply style
    // let single_style = theme.try_get("ui.jump.single").unwrap_or_else(|| {
    //     Style::default()
    //         .fg(Color::Rgb(0xff, 0x00, 0x7c))
    //         .add_modifier(Modifier::BOLD)
    // });
    // let multi_first_style = theme.try_get("ui.jump.multi-first").unwrap_or_else(|| {
    //     Style::default()
    //         .fg(Color::Rgb(0x00, 0xdf, 0xff))
    //         .add_modifier(Modifier::BOLD)
    // });
    // let multi_rest_style = theme
    //     .try_get("ui.jump.multi-rest")
    //     .unwrap_or_else(|| Style::default().fg(Color::Rgb(0x2b, 0x8d, 0xb3)));
}

pub fn show_key_annotations_with_callback<F>(
    ctx: &mut Context,
    annotations: Vec<JumpAnnotation>,
    on_key_press_callback: F,
) where
    F: FnOnce(&mut Context, KeyEvent) + 'static,
{
    log::error!("annotations: {:?}", annotations);
    // apply_dimming(ctx);
    let mut overlays_single: Vec<Overlay> = Vec::new();
    let mut overlays_multi_first: Vec<Overlay> = Vec::new();
    let mut overlays_multi_rest: Vec<Overlay> = Vec::new();
    for jump in annotations.into_iter() {
        if jump.keys.len() == 1 {
            overlays_single.push(Overlay {
                char_idx: jump.loc,
                grapheme: jump.keys.into(),
            });
            continue;
        }
        overlays_multi_first.push(Overlay {
            char_idx: jump.loc,
            grapheme: jump.keys.chars().next().unwrap().to_string().into(),
        });
        for (i, c) in (1..jump.keys.len()).zip(jump.keys.chars().skip(1)) {
            overlays_multi_rest.push(Overlay {
                char_idx: jump.loc + i,
                grapheme: c.to_string().into(),
            });
        }
    }
    let mut view = view_mut!(ctx.editor);
    overlays_single.sort_by_key(|overlay| overlay.char_idx);
    overlays_multi_first.sort_by_key(|overlay| overlay.char_idx);
    overlays_multi_rest.sort_by_key(|overlay| overlay.char_idx);
    view.visual_jump_labels[0] = overlays_single.into();
    view.visual_jump_labels[1] = overlays_multi_first.into();
    view.visual_jump_labels[2] = overlays_multi_rest.into();
    log::error!("Overlays[0]:\n{:?}", view.visual_jump_labels[0]);
    log::error!("Overlays[1]:\n{:?}", view.visual_jump_labels[1]);
    log::error!("Overlays[2]:\n{:?}", view.visual_jump_labels[2]);
    ctx.on_next_key(on_key_press_callback);
}
