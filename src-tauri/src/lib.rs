mod error;
mod state;
mod save;

use chrono::FixedOffset;
use tauri::{ipc::Response, Manager, State};
use tokio::sync::RwLock;
use state::*;
use error::InvokeError;
use tracing::{error, info, warn};
use base64::prelude::*;
use uuid::Uuid;

type InvokeResult<T> = Result<T, InvokeError>;

const JST_OFFSET: i32 = 9 * 60 * 60;

#[inline]
fn base_url() -> &'static str {
    if cfg!(debug_assertions) {
        option_env!("BASE_API_URL").expect("env for api url is not set")
    } else {
        env!("BASE_API_URL")
    }
}

#[inline]
fn jst() -> InvokeResult<FixedOffset> {
    match FixedOffset::east_opt(JST_OFFSET) {
        Some(jst) => Ok(jst),
        None => Err(InvokeError::Chrono)
    }
}

#[inline]
fn print_err<E>(err: E) -> E
where E: std::error::Error {
    error!("{err}");
    err
}

#[tauri::command]
async fn exists_auth(st_admin: State<'_, RwLock<CacheAdmin>>) -> InvokeResult<bool> {
    warn!("saving file is not implemented !!");

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
    warn!("authentication is not implemented !!");

    if org_id.is_empty() || admin_id.is_empty() || admin_pw.is_empty() {
        warn!("empty input");
        return Err(InvokeError::Input);
    }

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
    let mut st_users = st_users.write().await;

    if st_users.has(&st_admin.org_id) {
        let json = serde_json::to_string(&st_users.users).map_err(print_err)?;
        return Ok(Response::new(json));
    }

    let url = format!("{}/{}/view/?q=users", base_url(), st_admin.org_id);    
    info!("requesting to {url}");
    let mut users = reqwest::get(url).await.map_err(print_err)?
        .json::<Vec<User>>().await.map_err(print_err)?;

    let jst = jst().map_err(print_err)?;
    users.iter_mut().for_each(|u| u.created_at = u.created_at.with_timezone(&jst));
    users.sort_unstable_by_key(|u| u.created_at);

    let json = serde_json::to_string(&users).map_err(print_err)?;

    st_users.users = users;
    st_users.org_id = st_admin.org_id.clone();
    
    Ok(Response::new(json))
}

#[tauri::command]
async fn set_query_user(st_query: State<'_, RwLock<CacheQuery>>, user: String) 
-> InvokeResult<()> {
    if user.is_empty() {
        warn!("empty input");
        return Err(InvokeError::Input);
    }

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
    if q.is_empty() || y.is_empty() || m.is_empty() {
        warn!("empty input");
        return Err(InvokeError::Input);
    }

    let mut st_query = st_query.write().await;
    st_query.q = q;
    st_query.y = y;
    st_query.m = m;
    
    Ok(())
}

#[tauri::command]
async fn load_calls(
    st_admin: State<'_, RwLock<CacheAdmin>>,
    st_query: State<'_, RwLock<CacheQuery>>,
    st_calls: State<'_, RwLock<CacheCalls>>
) -> InvokeResult<Response> {
    let st_admin = st_admin.read().await;
    let st_query = st_query.read().await;
    let mut st_calls = st_calls.write().await;

    if &st_query.q != "calls" {
        error!("inconsistent cache");
        return Err(InvokeError::Internal);
    }

    if st_calls.has(&st_query.user, &st_query.y, &st_query.m) {
        let json =serde_json::to_string(&st_calls.calls).map_err(print_err)?;
        return Ok(Response::new(json));
    }

    let user_id = BASE64_STANDARD.decode(st_query.user.as_str()).map_err(print_err)?;
    let user_id = Uuid::from_slice(&user_id).map_err(print_err)?;

    let url = format!(
        "{}/{}/view/?q=calls&y={}&m={}&user={}",
        base_url(),
        st_admin.org_id,
        st_query.y,
        st_query.m,
        user_id.to_string()
    );
    info!("requesting to {url}");
    let mut calls = reqwest::get(url).await.map_err(print_err)?
        .json::<Calls>().await.map_err(print_err)?;
        
    let jst = jst().map_err(print_err)?;
    calls.morning_calls.iter_mut()
        .for_each(|c| c.created_at = c.created_at.with_timezone(&jst));
    calls.morning_calls.sort_unstable_by_key(|c| c.created_at);
    calls.evening_calls.iter_mut()
        .for_each(|c| c.created_at = c.created_at.with_timezone(&jst));
    calls.evening_calls.sort_unstable_by_key(|c| c.created_at);

    let json = serde_json::to_string(&calls).map_err(print_err)?;

    st_calls.calls = calls;

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
            set_query_user,
            set_query_qym,
            load_calls
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
