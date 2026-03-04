mod icon;
mod tray;

/// The self-service portal URL opened when the user clicks "Open Self-Service Portal".
///
/// TODO: Confirm final URL with the team.
const PORTAL_URL: &str = "https://itanywhere.halopsa.com/portal/";

fn main() {
    let icon = icon::load_icon();
    tray::run(icon, PORTAL_URL);
}
