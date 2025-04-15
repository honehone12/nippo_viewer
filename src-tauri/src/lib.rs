mod error;
mod state;

use tauri::{ipc::Response, Manager};
use tokio::sync::Mutex;
use state::AdminState;
use error::InvokeError;

type InvokeResult<T> = Result<T, InvokeError>;

#[tauri::command]
async fn exists_auth() -> InvokeResult<bool> {
    tracing::warn!("saving file is not implemented !!");

    Ok(false)
}

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
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            exists_auth,
            admin_auth
        ])
        .setup(|app| {
            app.manage(Mutex::new(AdminState::default()));
            Ok(())
        })
        .run(tauri::generate_context!())?;
    Ok(())
}
