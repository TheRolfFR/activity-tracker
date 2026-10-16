macro_rules! resource {
    ($path:tt) => {
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), $path))
    };
}

pub mod fonts {
    use iced::{Font, font::*};

    pub static SEGOE_UI_REGULAR: &[u8] = resource!("/assets/segoeui.ttf");
    pub static SEGOE_UI_SEMI_BOLD: &[u8] = resource!("/assets/seguisb.ttf");
    pub static SEGOE_UI_BOLD: &[u8] = resource!("/assets/segoeuib.ttf");
    pub static SEGOE_UI_LIGHT: &[u8] = resource!("/assets/segoeuil.ttf");

    pub const REGULAR: Font = Font {
        family: Family::Name("Segoe UI"),
        weight: Weight::Normal,
        stretch: Stretch::Normal,
        style: Style::Normal,
    };
    pub const SEMIBOLD: Font = Font {
        family: Family::Name("Segoe UI"),
        weight: Weight::Semibold,
        stretch: Stretch::Normal,
        style: Style::Normal,
    };
    pub const BOLD: Font = Font {
        family: Family::Name("Segoe UI"),
        weight: Weight::Bold,
        stretch: Stretch::Normal,
        style: Style::Normal,
    };
}

pub mod images {
    pub const ICON: &[u8] = resource!("/icons/32x32.png");
}
