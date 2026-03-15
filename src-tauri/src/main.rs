#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Suppress GTK3/WebKit theme compatibility warnings from the system dark theme
    std::env::set_var("GTK_THEME", "Adwaita");
    luzumi_lib::run()
}
