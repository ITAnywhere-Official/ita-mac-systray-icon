use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIconBuilder,
};

/// The self-service portal URL opened when the user clicks "Open Self-Service Portal".
///
/// TODO: Confirm final URL with the team.
const PORTAL_URL: &str = "https://itanywhere.halopsa.com/portal/";

fn main() {
    let event_loop = EventLoopBuilder::new().build();

    // Build the right-click / click menu
    let menu = Menu::new();
    let open_portal = MenuItem::new("Open Self-Service Portal", true, None);
    let quit = MenuItem::new("Quit", true, None);
    menu.append(&open_portal).expect("failed to add menu item");
    menu.append(&quit).expect("failed to add menu item");

    // Load the tray icon image
    let icon = load_icon();

    // Build the tray icon
    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("IT Anywhere - Self-Service Portal")
        .with_icon(icon)
        .build()
        .expect("failed to build tray icon");

    // Run the event loop
    let menu_channel = MenuEvent::receiver();
    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Ok(event) = menu_channel.try_recv() {
            if event.id() == open_portal.id() {
                let _ = open::that(PORTAL_URL);
            } else if event.id() == quit.id() {
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}

/// Loads the tray icon from the embedded PNG bytes.
///
/// The icon should be a 44x44 pixel PNG (2x retina) for macOS menu bar.
/// For now, this generates a simple placeholder icon.
fn load_icon() -> tray_icon::Icon {
    // Placeholder: 22x22 solid icon (will be replaced with a proper branded icon)
    let size = 22u32;
    let rgba = vec![0u8; (size * size * 4) as usize]
        .chunks_exact_mut(4)
        .flat_map(|_| [30, 136, 229, 255]) // A blue pixel (RGBA)
        .collect::<Vec<u8>>();
    tray_icon::Icon::from_rgba(rgba, size, size).expect("failed to create icon")
}
