/// Icon loading for the macOS menu bar.
///
/// Loads the ITA logo from a PNG asset embedded at compile time.
/// The PNG is generated from the vector SVG source in `assets/icon.svg`
/// using `resvg`.

/// The 44×44 (@2x Retina) icon embedded at compile time.
///
/// The 44×44 size is preferred because macOS will downscale it for
/// non-Retina displays, which produces better results than upscaling
/// a 22×22 image on Retina screens.
const ICON_PNG: &[u8] = include_bytes!("../assets/icon-44x44.png");

/// Loads the embedded ITA logo icon.
pub fn load_icon() -> tray_icon::Icon {
    let img = image::load_from_memory_with_format(ICON_PNG, image::ImageFormat::Png)
        .expect("failed to decode embedded icon PNG")
        .into_rgba8();
    let (width, height) = img.dimensions();
    tray_icon::Icon::from_rgba(img.into_raw(), width, height)
        .expect("failed to create icon from PNG data")
}