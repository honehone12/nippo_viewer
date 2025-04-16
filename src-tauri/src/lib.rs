mod error;
mod state;

use chrono::FixedOffset;
use tauri::{ipc::Response, Manager, State};
use tokio::sync::RwLock;
use state::*;
use error::InvokeError;
use tracing::{info, error};

type InvokeResult<T> = Result<T, InvokeError>;

const JST_OFFSET: i32 = 9 * 60 * 60;

#[tauri::command]
async fn exists_auth(st_admin: State<'_, RwLock<CacheAdmin>>) -> InvokeResult<bool> {
    tracing::warn!("saving file is not implemented !!");

    let mut st_admin = st_admin.write().await;

    Ok(false)
}

#[tauri::command]
async fn admin_auth(
    st_admin: State<'_, RwLock<CacheAdmin>>,
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
    st_admin: State<'_, RwLock<CacheAdmin>>, 
    st_users: State<'_, RwLock<CacheUsers>>,
) -> InvokeResult<Response> {
    let st_admin = st_admin.read().await;
    let org_id = st_admin.org_id.clone();
    let mut st_users = st_users.write().await;

    if st_users.has(&org_id) {
        match serde_json::to_string(&st_users.users) {
            Ok(s) => return Ok(Response::new(s)),
            Err(e) => {
                error!("{e}");
                return Err(InvokeError::JsonError);
            }
        }
    }

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
    users.sort_unstable_by_key(|u| u.created_at);

    let json = match serde_json::to_string(&users) {
        Ok(s) => s,
        Err(e) => {
            error!("{e}");
            return Err(InvokeError::JsonError);
        }
    };

    st_users.users = users;
    st_users.org_id = org_id;

    Ok(Response::new(json))
}

#[tauri::command]
async fn set_query_user(st_query: State<'_, RwLock<CacheQuery>>, user: String) 
-> InvokeResult<()> {
    let mut st_query = st_query.write().await;
    st_query.user = user;

    Ok(())
}

#[tauri::command]
async fn set_query_qym(
    st_query: State<'_, RwLock<CacheQuery>>,
    q: String,
    y: String,
    m: String
) -> InvokeResult<()> {
    let mut st_query = st_query.write().await;
    st_query.q = q;
    st_query.y = y;
    st_query.m = m;
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            exists_auth,
            admin_auth,
            load_users,
            set_query_user,
            set_query_qym
        ])
        .setup(|app| {
            app.manage(RwLock::new(CacheAdmin::default()));
            app.manage(RwLock::new(CacheUsers::default()));
            app.manage(RwLock::new(CacheQuery::default()));
            app.manage(RwLock::new(CacheCalls::default()));
            app.manage(RwLock::new(CacheDailyReports::default()));
            app.manage(RwLock::new(CacheDailyReportPrint::default()));
            Ok(())
        })
        .run(tauri::generate_context!())?;
    Ok(())
}
