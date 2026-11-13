//! Installed Windows symbols, with the existing SVGs as a per-glyph fallback.
//! No Microsoft font is bundled or redistributed.
use super::Icon;
use egui::{Color32, FontDefinitions, Rect, Response, Ui, Widget};
#[cfg(windows)]
use egui::{Sense, Vec2};

#[cfg(windows)]
const FAMILY: &str = "zapfast-windows-symbols";

pub(super) fn install(fonts: &mut FontDefinitions) {
    #[cfg(windows)]
    {
        use std::sync::{Arc, OnceLock};
        static FONT: OnceLock<Option<Vec<u8>>> = OnceLock::new();
        let bytes = FONT.get_or_init(|| {
            let directory = std::path::PathBuf::from(std::env::var_os("WINDIR")?).join("Fonts");
            ["SegoeIcons.ttf", "Segoe Fluent Icons.ttf", "segmdl2.ttf"]
                .into_iter()
                .find_map(|name| std::fs::read(directory.join(name)).ok())
        });
        if let Some(bytes) = bytes {
            fonts.font_data.insert(
                FAMILY.into(),
                Arc::new(egui::FontData::from_owned(bytes.clone())),
            );
            fonts
                .families
                .insert(egui::FontFamily::Name(FAMILY.into()), vec![FAMILY.into()]);
        }
    }
    #[cfg(not(windows))]
    let _ = fonts;
}

/// Keeps each original icon's allocation, pointer sense and accessibility.
pub struct IconImage {
    icon: Icon,
    color: Color32,
    size: f32,
}

impl Icon {
    pub fn fluent_image(self, color: Color32, size: f32) -> IconImage {
        IconImage {
            icon: self,
            color,
            size,
        }
    }
}

impl IconImage {
    pub fn paint_at(&self, ui: &Ui, rect: Rect) {
        #[cfg(windows)]
        if let Some(glyph) = glyph(self.icon) {
            let font = egui::FontId::new(self.size, egui::FontFamily::Name(FAMILY.into()));
            let available = ui.fonts_mut(|fonts| {
                fonts.families().contains(&font.family) && fonts.has_glyph(&font, glyph)
            });
            if available {
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    glyph,
                    font,
                    self.color,
                );
                if matches!(self.icon, Icon::VideoOff | Icon::WifiOff | Icon::ShieldOff) {
                    let offset = egui::vec2(self.size * 0.36, self.size * 0.36);
                    ui.painter().line_segment(
                        [rect.center() - offset, rect.center() + offset],
                        egui::Stroke::new((self.size / 12.0).max(1.2), self.color),
                    );
                }
                return;
            }
        }
        self.icon.image(self.color, self.size).paint_at(ui, rect);
    }
}

impl Widget for IconImage {
    fn ui(self, ui: &mut Ui) -> Response {
        #[cfg(not(windows))]
        {
            self.icon.image(self.color, self.size).ui(ui)
        }
        #[cfg(windows)]
        {
            let (rect, response) = ui.allocate_exact_size(Vec2::splat(self.size), Sense::hover());
            response.widget_info(|| egui::WidgetInfo::new(egui::WidgetType::Image));
            if ui.is_rect_visible(rect) {
                self.paint_at(ui, rect);
            }
            response
        }
    }
}

// Stable symbols documented in Microsoft's Segoe Fluent Icons table.
// Specialized delivery marks and unavailable symbols
// keep their original artwork rather than changing the meaning of an action.
#[cfg(windows)]
fn glyph(icon: Icon) -> Option<char> {
    Some(match icon {
        Icon::Archive => '\u{e7b8}',
        Icon::ArrowDown => '\u{e74b}',
        Icon::ArrowLeft => '\u{e72b}',
        Icon::Ban => '\u{e733}',
        Icon::Bell => '\u{ea8f}',
        Icon::BellOff => '\u{e7ed}',
        Icon::Calendar => '\u{e787}',
        Icon::Check => '\u{e73e}',
        Icon::ChevronDown => '\u{e70d}',
        Icon::ChevronLeft => '\u{e76b}',
        Icon::ChevronRight => '\u{e76c}',
        Icon::ChevronUp => '\u{e70e}',
        Icon::ListChecks => '\u{e9d5}',
        Icon::CircleAlert => '\u{e7ba}',
        Icon::CircleCheck => '\u{e930}',
        Icon::CircleX => '\u{ea39}',
        Icon::Clock => '\u{e917}',
        Icon::Contact | Icon::User => '\u{e77b}',
        Icon::Copy => '\u{e8c8}',
        Icon::Download => '\u{e896}',
        Icon::Timer => '\u{e916}',
        Icon::Ellipsis => '\u{e712}',
        Icon::Eraser => '\u{e75c}',
        Icon::ExternalLink => '\u{e8a7}',
        Icon::Eye => '\u{e890}',
        Icon::EyeOff => '\u{ed1a}',
        Icon::FileText => '\u{e8a5}',
        Icon::Forward => '\u{e72a}',
        Icon::Gif => '\u{f4a9}',
        Icon::Heart => '\u{eb51}',
        Icon::Image => '\u{eb9f}',
        Icon::Info => '\u{e946}',
        Icon::Keyboard => '\u{e765}',
        Icon::Link => '\u{e71b}',
        Icon::Lock => '\u{e72e}',
        Icon::LockOpen => '\u{e785}',
        Icon::LogOut => '\u{f3b1}',
        Icon::MapPin => '\u{e707}',
        Icon::Maximize => '\u{e740}',
        Icon::MessageCircle => '\u{e8f2}',
        Icon::Mic => '\u{e720}',
        Icon::MicOff => '\u{f781}',
        Icon::Minimize => '\u{e73f}',
        Icon::Minus => '\u{e738}',
        Icon::Monitor => '\u{e7f4}',
        Icon::Moon => '\u{e708}',
        Icon::Music => '\u{e8d6}',
        Icon::PanelLeft => '\u{e700}',
        Icon::Paperclip => '\u{e723}',
        Icon::Pause => '\u{e769}',
        Icon::Pencil | Icon::SquarePen => '\u{e70f}',
        Icon::Phone => '\u{e717}',
        Icon::PhoneIncoming => '\u{e77e}',
        Icon::PhoneOutgoing => '\u{eac2}',
        Icon::PhoneOff => '\u{f405}',
        Icon::Pin => '\u{e718}',
        Icon::PinOff => '\u{e77a}',
        Icon::Play => '\u{e768}',
        Icon::Plus => '\u{e710}',
        Icon::QrCode => '\u{ed14}',
        Icon::Refresh => '\u{e72c}',
        Icon::RotateLeft => '\u{e80d}',
        Icon::RotateRight => '\u{e80c}',
        Icon::Reply => '\u{e97a}',
        Icon::Search => '\u{e721}',
        Icon::Send => '\u{e724}',
        Icon::Settings => '\u{e713}',
        Icon::Shield | Icon::ShieldOff => '\u{ea18}',
        Icon::Smartphone => '\u{e8ea}',
        Icon::Smile => '\u{e899}',
        Icon::Star => '\u{e734}',
        Icon::StarFill => '\u{e735}',
        Icon::StarOff => '\u{e8d9}',
        Icon::Sticker => '\u{f4aa}',
        Icon::Sun => '\u{e706}',
        Icon::Tag => '\u{e8ec}',
        Icon::Trash => '\u{e74d}',
        Icon::UserPlus => '\u{e8fa}',
        Icon::UserMinus => '\u{f69b}',
        Icon::Users => '\u{e716}',
        Icon::Video | Icon::VideoOff => '\u{e714}',
        Icon::Volume2 => '\u{e767}',
        Icon::VolumeX => '\u{e74f}',
        Icon::WifiOff => '\u{e701}',
        Icon::X => '\u{e711}',
        _ => return None,
    })
}
