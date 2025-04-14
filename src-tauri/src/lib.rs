pub mod error;

use std::error::Error;
use tauri::ipc::Response;

type InvokeResult<T> = Result<T, error::InvokeError>;

#[tauri::command]
async fn admin_auth(
    org_id: &str,
    admin_id: &str,
    admin_pw: &str
) -> InvokeResult<()> {
    tracing::warn!("authentication is not implemented !!");

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            admin_auth
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
