use std::borrow::Cow;

use gdk_pixbuf::{
    glib::{FromVariant, Variant, VariantTy},
    prelude::{StaticVariantType, ToVariant},
};
use hex_color::HexColor;
use pangocairo::pango::SCALE;
use relm4::gtk::gdk::RGBA;

use crate::configuration::APP_CONFIG;

#[derive(Clone, Copy, Debug, Default)]
pub struct Style {
    pub color: Color,
    pub size: Size,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum Size {
    Small = 0,
    #[default]
    Medium = 1,
    Large = 2,
}

impl Default for Color {
    fn default() -> Self {
        Self::orange()
    }
}

impl StaticVariantType for Color {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        Cow::Borrowed(VariantTy::TUPLE)
    }
}
impl ToVariant for Color {
    fn to_variant(&self) -> Variant {
        (self.r, self.g, self.b, self.a).to_variant()
    }
}

impl FromVariant for Color {
    fn from_variant(variant: &Variant) -> Option<Self> {
        <(u8, u8, u8, u8)>::from_variant(variant).map(|(r, g, b, a)| Self { r, g, b, a })
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_gdk(rgba: RGBA) -> Self {
        Self::new(
            (rgba.red() * 255.0) as u8,
            (rgba.green() * 255.0) as u8,
            (rgba.blue() * 255.0) as u8,
            (rgba.alpha() * 255.0) as u8,
        )
    }

    pub fn orange() -> Self {
        Self::new(240, 147, 43, 255)
    }
    pub fn red() -> Self {
        Self::new(235, 77, 75, 255)
    }
    pub fn green() -> Self {
        Self::new(106, 176, 76, 255)
    }
    pub fn blue() -> Self {
        Self::new(34, 166, 179, 255)
    }
    pub fn cove() -> Self {
        Self::new(19, 15, 64, 255)
    }

    pub fn pink() -> Self {
        Self::new(200, 37, 184, 255)
    }

    pub fn to_rgba_f64(self) -> (f64, f64, f64, f64) {
        (
            (self.r as f64) / 255.0,
            (self.g as f64) / 255.0,
            (self.b as f64) / 255.0,
            (self.a as f64) / 255.0,
        )
    }
    pub fn to_rgba_u32(self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
    }
}

impl From<RGBA> for Color {
    fn from(value: RGBA) -> Self {
        Self::new(
            (value.red() * 255.0) as u8,
            (value.green() * 255.0) as u8,
            (value.blue() * 255.0) as u8,
            (value.alpha() * 255.0) as u8,
        )
    }
}

impl From<Color> for RGBA {
    fn from(color: Color) -> Self {
        Self::new(
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
            color.a as f32 / 255.0,
        )
    }
}

impl From<HexColor> for Color {
    fn from(value: HexColor) -> Self {
        Self::new(value.r, value.g, value.b, value.a)
    }
}

impl StaticVariantType for Size {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        Cow::Borrowed(VariantTy::UINT32)
    }
}

impl ToVariant for Size {
    fn to_variant(&self) -> Variant {
        Variant::from(*self as u32)
    }
}

impl FromVariant for Size {
    fn from_variant(variant: &Variant) -> Option<Self> {
        variant.get::<u32>().and_then(|v| match v {
            0 => Some(Size::Small),
            1 => Some(Size::Medium),
            2 => Some(Size::Large),
            _ => None,
        })
    }
}

impl Size {
    pub fn to_text_size(self) -> i32 {
        let size_factor = APP_CONFIG.read().annotation_size_factor();

        match self {
            Size::Small => (12.0 * SCALE as f64 * size_factor) as i32,
            Size::Medium => (18.0 * SCALE as f64 * size_factor) as i32,
            Size::Large => (32.0 * SCALE as f64 * size_factor) as i32,
        }
    }

    pub fn to_line_width(self) -> f64 {
        let size_factor = APP_CONFIG.read().annotation_size_factor();

        match self {
            Size::Small => 2.0 * size_factor,
            Size::Medium => 3.0 * size_factor,
            Size::Large => 5.0 * size_factor,
        }
    }

    pub fn to_blur_factor(self) -> f64 {
        let size_factor = APP_CONFIG.read().annotation_size_factor();
        match self {
            Size::Small => 6.0 * size_factor,
            Size::Medium => 10.0 * size_factor,
            Size::Large => 20.0 * size_factor,
        }
    }
}
