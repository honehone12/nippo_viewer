mod error;
mod state;
mod save;

use chrono::FixedOffset;
use tauri::{ipc::Response, Manager, State};
use tokio::{fs, sync::RwLock};
use tracing::{error, info, warn};
use base64::prelude::*;
use uuid::Uuid;
use state::*;
use save::{persistent_file_path, Save};
use error::{InvokeError, InvokeResult, print_err};

#[inline]
fn base_url() -> &'static str {
    #[cfg(debug_assertions)]
    return option_env!("BASE_API_URL").expect("env for api url is not set");
    #[cfg(not(debug_assertions))]
    return env!("BASE_API_URL");
}

#[inline]
fn jst() -> InvokeResult<FixedOffset> {
    match FixedOffset::east_opt(9 * 60 * 60) {
        Some(jst) => Ok(jst),
        None => Err(InvokeError::Chrono)
    }
}

#[tauri::command]
async fn exists_auth(st_admin: State<'_, RwLock<CacheAdmin>>) -> InvokeResult<bool> {
    let path = persistent_file_path()?;
    if !fs::try_exists(&path).await.map_err(print_err)? {
        _ = fs::File::create(path).await.map_err(print_err)?;
        return Ok(false);
    }

    warn!("check saved auth is valid here !!");

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
    if org_id.is_empty() || admin_id.is_empty() || admin_pw.is_empty() {
        warn!("empty input");
        return Err(InvokeError::Input);
    }

    warn!("do authentication here !!");

    let tkn = String::from("test-token-999"); 
    let admin = CacheAdmin{ 
        org_id, 
        tkn
    };
    let json = serde_json::to_string(&admin).map_err(print_err)?;
    let save = Save::from_text(json)?;
    let json = serde_json::to_string(&save).map_err(print_err)?;
    let path = persistent_file_path()?;
    fs::write(path, json).await.map_err(print_err)?;

    let mut st_admin = st_admin.write().await;
    *st_admin = admin;

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
        "{}/{}/view?q=calls&y={}&m={}&user={}",
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
    st_calls.y = st_query.y.clone();
    st_calls.m = st_query.m.clone();
    st_calls.user = st_query.user.clone();

    Ok(Response::new(json))
}

#[tauri::command]
async fn load_reports(
    st_admin: State<'_, RwLock<CacheAdmin>>,
    st_query: State<'_, RwLock<CacheQuery>>,
    st_reports: State<'_, RwLock<CacheDailyReports>>
) -> InvokeResult<Response> {
    let st_admin = st_admin.read().await;
    let st_query = st_query.read().await;
    let mut st_reports = st_reports.write().await;

    if &st_query.q != "reports" {
        error!("inconsistent cache");
        return Err(InvokeError::Internal);
    }

    if st_reports.has(&st_query.user, &st_query.y, &st_query.m) {
        let json = serde_json::to_string(&st_reports.reports).map_err(print_err)?;
        return Ok(Response::new(json));
    }

    let user_id = BASE64_STANDARD.decode(st_query.user.as_str()).map_err(print_err)?;
    let user_id = Uuid::from_slice(&user_id).map_err(print_err)?;

    let url = format!(
        "{}/{}/view?q={}&y={}&m={}&user={}",
        base_url(),
        st_admin.org_id,
        st_query.q,
        st_query.y,
        st_query.m,
        user_id.to_string()
    );
    info!("requesting to {url}");
    let mut reports = reqwest::get(url).await.map_err(print_err)?
        .json::<Vec<DailyReport>>().await.map_err(print_err)?;

    let jst = jst().map_err(print_err)?;
    reports.iter_mut().for_each(|r| {
        r.created_at = r.created_at.with_timezone(&jst);
        r.updated_at = r.updated_at.with_timezone(&jst);
    });
    reports.sort_unstable_by_key(|r| r.created_at);

    let json = serde_json::to_string(&reports).map_err(print_err)?;

    st_reports.reports = reports;
    st_reports.y = st_query.y.clone();
    st_reports.m = st_query.m.clone();
    st_reports.user = st_query.user.clone();

    Ok(Response::new(json))
}

#[tauri::command]
async fn load_print(
    st_admin: State<'_, RwLock<CacheAdmin>>,
    st_query: State<'_, RwLock<CacheQuery>>,
    st_print: State<'_, RwLock<CacheDailyReportPrint>>    
) -> InvokeResult<Response> {
    let st_admin = st_admin.read().await;
    let st_query = st_query.read().await;
    let mut st_print = st_print.write().await;

    Err(InvokeError::Internal)
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
            load_calls,
            load_reports,
            load_print
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
