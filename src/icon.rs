/// Programmatic icon generation for the macOS menu bar.
///
/// Generates a bullseye icon (concentric circles) inspired by the IT Anywhere logo.
/// The icon is monochrome black with alpha transparency, making it a macOS
/// "template image" that automatically adapts to light and dark mode.

/// Creates a bullseye icon of the given size in pixels.
///
/// The bullseye consists of:
/// - A solid inner dot at the centre
/// - A ring (annulus) around it with a gap between
///
/// All pixels are black (`0, 0, 0`) with alpha derived from distance to circle
/// edges, providing basic anti-aliasing for a smooth appearance.
pub fn create_bullseye_icon(size: u32) -> tray_icon::Icon {
    let centre = size as f64 / 2.0;

    // Radii scaled proportionally to the icon size
    let inner_radius = size as f64 * 0.14;
    let ring_inner = size as f64 * 0.30;
    let ring_outer = size as f64 * 0.45;

    let mut rgba = Vec::with_capacity((size * size * 4) as usize);

    for y in 0..size {
        for x in 0..size {
            let dx = x as f64 + 0.5 - centre;
            let dy = y as f64 + 0.5 - centre;
            let dist = (dx * dx + dy * dy).sqrt();

            let alpha = if dist <= inner_radius {
                // Inner dot — solid with anti-aliased edge
                edge_alpha(dist, inner_radius)
            } else if dist >= ring_inner && dist <= ring_outer {
                // Outer ring — solid with anti-aliased inner and outer edges
                let outer_aa = edge_alpha(dist, ring_outer);
                let inner_aa = edge_alpha(ring_inner, dist);
                outer_aa.min(inner_aa)
            } else {
                0.0
            };

            rgba.push(0); // R — black
            rgba.push(0); // G
            rgba.push(0); // B
            rgba.push((alpha * 255.0) as u8); // A
        }
    }

    tray_icon::Icon::from_rgba(rgba, size, size).expect("failed to create bullseye icon")
}

/// Returns an alpha value for anti-aliasing at circle edges.
///
/// When `dist` is well inside `radius`, returns 1.0 (fully opaque).
/// When `dist` is well outside `radius`, returns 0.0 (fully transparent).
/// Within a 1-pixel transition band, returns a smooth interpolation.
fn edge_alpha(dist: f64, radius: f64) -> f64 {
    (radius - dist + 0.5).clamp(0.0, 1.0)
}
