// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .init();
    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();
    
    nippo_viewer_lib::run()
}
