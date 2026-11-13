//! Presentation only. No account, protocol, settings format or action changes.
use super::{BubbleStyle, Palette};
use egui::Color32;

fn active_id() -> egui::Id {
    egui::Id::new("windows-fluent-material")
}

pub(super) fn set_active(ctx: &egui::Context, active: bool) {
    ctx.data_mut(|data| data.insert_temp(active_id(), active));
}

/// Adds in-app Acrylic-style frost behind measured popup backgrounds. This
/// touches paint shapes only: popup layout, focus, hits and actions stay intact.
/// The main window continues using the original native DWM Mica implementation.
pub(super) fn install(ctx: &egui::Context) {
    ctx.on_end_pass(
        "windows-popup-material",
        std::sync::Arc::new(|ctx| {
            if !ctx.data(|data| data.get_temp::<bool>(active_id()).unwrap_or(false)) {
                return;
            }
            if !egui::Popup::is_any_open(ctx) {
                return;
            }
            let layers: Vec<_> = ctx.memory(|memory| memory.layer_ids().collect());
            ctx.graphics_mut(|graphics| {
                for layer in layers {
                    if let Some(list) = graphics.get_mut(layer) {
                        let count = list.all_entries().len();
                        for index in 0..count {
                            list.mutate_shape(egui::layers::ShapeIdx(index), |entry| {
                                frost_popup(&mut entry.shape)
                            });
                        }
                    }
                }
            });
        }),
    );
}

fn frost_popup(shape: &mut egui::Shape) {
    match shape {
        egui::Shape::Vec(shapes) => {
            for child in shapes {
                frost_popup(child);
            }
        }
        egui::Shape::Rect(rect) => {
            let dark = Color32::from_rgba_unmultiplied(44, 44, 44, 240);
            let light = Color32::from_rgba_unmultiplied(252, 252, 252, 240);
            if rect.fill == dark || rect.fill == light {
                let background = rect.clone();
                *shape = egui::Shape::Vec(vec![
                    crate::ui::glass::rounded_shape(background.rect, 0.0, 8.0),
                    egui::Shape::Rect(background),
                ]);
            }
        }
        _ => {}
    }
}

/// Neutral Fluent surfaces, retaining ZapFast's green accent and identity.
pub(super) fn palette(mut p: Palette) -> Palette {
    let gray = |value| Color32::from_gray(value);
    if p.dark {
        p.window = gray(32);
        p.panel = gray(36);
        p.surface = gray(45);
        p.surface_hover = gray(55);
        p.surface_active = gray(64);
        p.outline = gray(65);
        p.text = gray(250);
        p.secondary = gray(198);
        p.dim = gray(178);
        p.overlay = gray(44);
        p.chat = gray(28);
        p.bubble_in = gray(45);
        p.bubble_out = Color32::from_rgb(25, 74, 61);
        // Keep text and button labels readable on the original green.
        p.accent = Color32::from_rgb(79, 212, 167);
        p.accent_hover = Color32::from_rgb(101, 224, 184);
        p.on_accent = gray(20);
    } else {
        p.window = gray(243);
        p.panel = gray(249);
        p.surface = gray(255);
        p.surface_hover = gray(239);
        p.surface_active = gray(229);
        p.outline = gray(222);
        p.text = gray(26);
        p.secondary = gray(84);
        p.dim = gray(103);
        p.overlay = gray(252);
        p.chat = gray(250);
        p.bubble_in = Color32::WHITE;
        p.bubble_out = Color32::from_rgb(221, 241, 231);
    }
    p.bubbles = BubbleStyle::Fluent;
    p
}
