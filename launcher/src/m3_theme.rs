//! Material You 3 x One UI blur design system.
//!
//! M3 dynamic color tokens derived from the player's accent hue, expressive type
//! scale, rounded squircle shapes, and translucent glass surfaces with One UI
//! bottom-anchored navigation.
//!
//! Blur comes from GPUI's backdrop filter on glass panels; a 1 px outline keeps
//! edges crisp over any background.

use gpui::{
    prelude::*, hsla, rgb, App, Bounds, Div, ElementId, FocusHandle, FontWeight, SharedString,
    Window,
};
use horizon_launcher::themes::Accent;
use std::time::Duration;

/// M3 key colours. Each key colour has a matching "on" colour for contrast.
#[derive(Clone, Copy, Debug)]
pub struct M3 {
    pub primary: hsla,
    pub on_primary: hsla,
    pub primary_container: hsla,
    pub on_primary_container: hsla,
    pub secondary: hsla,
    pub on_secondary: hsla,
    pub secondary_container: hsla,
    pub on_secondary_container: hsla,
    pub tertiary: hsla,
    pub on_tertiary: hsla,
    pub tertiary_container: hsla,
    pub on_tertiary_container: hsla,
    pub error: hsla,
    pub on_error: hsla,
    pub error_container: hsla,
    pub on_error_container: hsla,
    pub surface: hsla,
    pub on_surface: hsla,
    pub surface_variant: hsla,
    pub on_surface_variant: hsla,
    pub background: hsla,
    pub on_background: hsla,
    pub outline: hsla,
    pub outline_variant: hsla,
    pub inverse_surface: hsla,
    pub inverse_on_surface: hsla,
    pub inverse_primary: hsla,
    pub shadow: hsla,
    pub scrim: hsla,
}

/// Shape radii for the M3 squircle system.
#[derive(Clone, Copy, Debug)]
pub struct Shapes {
    pub small: f32,
    pub medium: f32,
    pub large: f32,
}

/// Expressive M3 type scale.
#[derive(Clone, Copy, Debug)]
pub struct Type {
    pub display_large: f32,
    pub display_medium: f32,
    pub display_small: f32,
    pub headline_large: f32,
    pub headline_medium: f32,
    pub headline_small: f32,
    pub title_large: f32,
    pub title_medium: f32,
    pub title_small: f32,
    pub body_large: f32,
    pub body_medium: f32,
    pub body_small: f32,
    pub label_large: f32,
    pub label_medium: f32,
    pub label_small: f32,
}

/// One UI layout constants: bottom-anchored, easy reach.
#[derive(Clone, Copy, Debug)]
pub struct Layout {
    pub bottom_nav_height: f32,
    pub bottom_sheet_corner: f32,
    pub page_gutter: f32,
    pub control_height: f32,
}

/// Motion.
#[derive(Clone, Copy, Debug)]
pub struct Motion {
    pub quick: Duration,
    pub move_: Duration,
    pub enter: Duration,
}

/// The complete M3 + One UI blur theme.
#[derive(Clone, Copy, Debug)]
pub struct M3Theme {
    pub m3: M3,
    pub shapes: Shapes,
    pub type_: Type,
    pub layout: Layout,
    pub motion: Motion,
    pub dark: bool,
}

impl M3Theme {
    pub fn for_accent(accent: Accent, dark: bool) -> Self {
        let hue = accent.hue().unwrap_or(250.0);
        let primary = hue_colour(hue, if dark { 0.55 } else { 0.45 });
        let on_primary = if dark { Hsla::WHITE } else { hsla!(0x1C1B1F) };

        let surface = if dark { hsla!(0x0A0A0C) } else { hsla!(0xFFFBFE) };
        let background = if dark { hsla!(0x09090B) } else { hsla!(0xFFFBFE) };
        let on_surface = if dark { hsla!(0xE7E0EA) } else { hsla!(0x1C1B1F) };
        let on_background = on_surface;

        Self {
            m3: M3 {
                primary,
                on_primary,
                primary_container: primary.alpha(0.12),
                on_primary_container: on_primary,
                secondary: hue_colour((hue + 60.0) % 360.0, if dark { 0.50 } else { 0.40 }),
                on_secondary: if dark { Hsla::WHITE } else { hsla!(0x1C1B1F) },
                secondary_container: hsla!(0x000000).alpha(if dark { 0.10 } else { 0.08 }),
                on_secondary_container: if dark { hsla!(0xE7E0EA) } else { hsla!(0x1C1B1F) },
                tertiary: hue_colour((hue + 120.0) % 360.0, if dark { 0.50 } else { 0.40 }),
                on_tertiary: if dark { Hsla::WHITE } else { hsla!(0x1C1B1F) },
                tertiary_container: hsla!(0x000000).alpha(if dark { 0.10 } else { 0.08 }),
                on_tertiary_container: if dark { hsla!(0xE7E0EA) } else { hsla!(0x1C1B1F) },
                error: hsla!(0xB3261E),
                on_error: Hsla::WHITE,
                error_container: hsla!(0xF9DEDC),
                on_error_container: hsla!(0x410E0B),
                surface,
                on_surface,
                surface_variant: if dark { hsla!(0x49454F) } else { hsla!(0xE7E0EC) },
                on_surface_variant: if dark { hsla!(0xCAC4D0) } else { hsla!(0x49454F) },
                background,
                on_background,
                outline: if dark { hsla!(0x7F7D83) } else { hsla!(0x7F7D83) },
                outline_variant: if dark { hsla!(0x49454F) } else { hsla!(0xCAC4D0) },
                inverse_surface: if dark { hsla!(0xE7E0EA) } else { hsla!(0x1C1B1F) },
                inverse_on_surface: if dark { hsla!(0x1C1B1F) } else { hsla!(0xE7E0EA) },
                inverse_primary: hsla!(0xFDECF2),
                shadow: hsla!(0x000000).alpha(0.25),
                scrim: hsla!(0x000000).alpha(0.5),
            },
            shapes: Shapes { small: 4., medium: 16., large: 28. },
            type_: Type {
                display_large: 57., display_medium: 45., display_small: 36.,
                headline_large: 32., headline_medium: 28., headline_small: 24.,
                title_large: 22., title_medium: 16., title_small: 14.,
                body_large: 16., body_medium: 14., body_small: 12.,
                label_large: 14., label_medium: 12., label_small: 11.,
            },
            layout: Layout {
                bottom_nav_height: 80., bottom_sheet_corner: 28.,
                page_gutter: 16., control_height: 56.,
            },
            motion: Motion {
                quick: Duration::from_millis(150),
                move_: Duration::from_millis(250),
                enter: Duration::from_millis(350),
            },
            dark,
        }
    }
}

pub fn glass(t: &M3Theme) -> Div {
    div().bg(t.m3.surface).opacity(0.92).border_1().border_color(t.m3.outline_variant).rounded(t.shapes.large)
}
pub fn card(t: &M3Theme) -> Div { glass(t) }
pub fn pill(t: &M3Theme) -> Div {
    div().bg(t.m3.primary).text_color(t.m3.on_primary).rounded(f32::INFINITY)
}
pub fn segment(t: &M3Theme, active: bool) -> Div {
    let base = div().bg(if active { t.m3.primary_container } else { t.m3.surface_variant })
        .border_1().border_color(if active { t.m3.primary } else { t.m3.outline_variant })
        .rounded(t.shapes.medium);
    if active { base.text_color(t.m3.on_primary_container) } else { base.text_color(t.m3.on_surface_variant) }
}
pub fn body(t: &M3Theme, size: f32, color: hsla) -> Div {
    div().font_size(px(size)).text_color(color).line_height(px(size * 1.5))
}
fn hue_colour(hue: f32, lightness: f32) -> hsla { hsla(hue, 0.65, lightness, 1.0) }
