mod error;
mod state;

use chrono::FixedOffset;
use tauri::{ipc::Response, Manager, State};
use tokio::sync::RwLock;
use state::{Admin, User};
use error::InvokeError;
use tracing::{info, error};

type InvokeResult<T> = Result<T, InvokeError>;

const JST_OFFSET: i32 = 9 * 60 * 60;

#[tauri::command]
async fn exists_auth(st_admin: State<'_, RwLock<Admin>>) -> InvokeResult<bool> {
    tracing::warn!("saving file is not implemented !!");

    let mut st_admin = st_admin.write().await;

    Ok(false)
}

#[tauri::command]
async fn admin_auth(
    st_admin: State<'_, RwLock<Admin>>,
    org_id: String,
    admin_id: String,
    admin_pw: String
) -> InvokeResult<()> {
    tracing::warn!("authentication is not implemented !!");

    let mut st_admin = st_admin.write().await;
    st_admin.org_id = org_id;

    Ok(())
}

#[tauri::command]
async fn load_users(
    st_admin: State<'_, RwLock<Admin>>, 
    st_user: State<'_, RwLock<Vec<User>>>,
) -> InvokeResult<Response> {
    let st_admin = st_admin.read().await;
    #[cfg(not(debug_assertions))]
    let base_url = env!("BASE_API_URL");
    #[cfg(debug_assertions)]
    let base_url = option_env!("BASE_API_URL")
        .expect("env for api url is not set");
    let url = format!("{base_url}/{}/view/?q=users", st_admin.org_id);
    
    info!("requesting to {url}");
    let res = match reqwest::get(url).await {
        Ok(res) => res.json::<Vec<User>>().await,
        Err(e) => {
            error!("{e}");
            return Err(InvokeError::HttpError);
        }
    };
    let mut users = match res {
        Ok(v) => v,
        Err(e) => {
            error!("{e}");
            return Err(InvokeError::HttpError);
        }
    };

    let Some(jst) = FixedOffset::east_opt(JST_OFFSET) else {
        error!("failed to handle jst offset");
        return Err(InvokeError::TimezoneError);
    };
    for u in users.iter_mut()  {
        u.created_at = u.created_at.with_timezone(&jst);
    }
    users.sort_unstable_by_key(|u| u.created_at);

    let mut st_user = st_user.write().await;
    *st_user = users;

    Ok(Response::new("".to_string()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            exists_auth,
            admin_auth,
            load_users,
        ])
        .setup(|app| {
            app.manage(RwLock::new(Admin::default()));
            app.manage(RwLock::new(Vec::<User>::new()));
            Ok(())
        })
        .run(tauri::generate_context!())?;
    Ok(())
}
