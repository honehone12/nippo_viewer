// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .with_line_number(true)
        .init();
    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .with_line_number(true)
        .init();
    
    nippo_viewer_lib::run()
}
