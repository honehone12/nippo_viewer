pub mod error;

use tauri::ipc::Response;
use tracing::warn;

type InvokeResult<T> = Result<T, error::InvokeError>;

#[tauri::command]
async fn admin_auth(
    org_id: &str,
    admin_id: &str,
    admin_pw: &str
) -> InvokeResult<()> {
    warn!("authentication is not implemented !!");

    Err(error::InvokeError::HttpRequestError)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if let Err(e) = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            admin_auth
        ])
        .run(tauri::generate_context!())
    {

        panic!("{e}");
    }
}
