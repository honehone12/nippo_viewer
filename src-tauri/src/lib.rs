mod error;
mod state;

use chrono::FixedOffset;
use tauri::{ipc::Response, Manager, State};
use tokio::sync::RwLock;
use state::{Admin, Users, User};
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
    st_users: State<'_, RwLock<Users>>,
) -> InvokeResult<Response> {
    let st_admin = st_admin.read().await;
    let org_id = st_admin.org_id.clone();
    #[cfg(not(debug_assertions))]
    let base_url = env!("BASE_API_URL");
    #[cfg(debug_assertions)]
    let base_url = option_env!("BASE_API_URL")
        .expect("env for api url is not set");
    let url = format!("{base_url}/{}/view/?q=users", org_id);
    
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
        error!("failed to create jst offset");
        return Err(InvokeError::ChronoError);
    };
    users.iter_mut().for_each(|u| u.created_at = u.created_at.with_timezone(&jst));
    // users.iter_mut().for_each(|u| u.updated_at = u.updated_at.with_timezone(&jst));
    // users.retain(|u| u.deleted_at.is_none());
    users.sort_unstable_by_key(|u| u.created_at);

    let json = match serde_json::to_string(&users) {
        Ok(s) => s,
        Err(e) => {
            error!("{e}");
            return Err(InvokeError::JsonError);
        }
    };

    let mut st_users = st_users.write().await;
    st_users.users = users;
    st_users.org_id = org_id;
    
    Ok(Response::new(json))
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
            app.manage(RwLock::new(Users::default()));
            Ok(())
        })
        .run(tauri::generate_context!())?;
    Ok(())
}
