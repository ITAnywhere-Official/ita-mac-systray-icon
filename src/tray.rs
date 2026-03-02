/// Tray icon construction and event loop.
///
/// Builds the macOS menu bar icon with its context menu and runs the
/// application event loop. The event loop blocks indefinitely and does
/// not return.

use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIconBuilder,
};

/// Constructs the tray icon and runs the event loop.
///
/// This function does not return — `event_loop.run()` takes ownership
/// of the thread.
pub fn run(icon: tray_icon::Icon, portal_url: &str) {
    let event_loop = EventLoopBuilder::new().build();

    let menu = Menu::new();
    let open_portal = MenuItem::new("Open Self-Service Portal", true, None);
    let quit = MenuItem::new("Quit", true, None);
    menu.append(&open_portal).expect("failed to add menu item");
    menu.append(&quit).expect("failed to add menu item");

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("IT Anywhere - Self-Service Portal")
        .with_icon(icon)
        .build()
        .expect("failed to build tray icon");

    let url = portal_url.to_owned();
    let menu_channel = MenuEvent::receiver();

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Ok(event) = menu_channel.try_recv() {
            if event.id() == open_portal.id() {
                let _ = open::that(&url);
            } else if event.id() == quit.id() {
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}
