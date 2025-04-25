mod error;
mod save;
mod state;

use base64::prelude::*;
use chrono::FixedOffset;
use dotenvy_macro::dotenv;
use error::{print_err, InvokeError, InvokeResult};
use save::{persistent_file_path, Save};
use state::*;
use tauri::{ipc::Response, AppHandle, Emitter, Manager, State, Url};
use tauri_plugin_deep_link::DeepLinkExt;
use tokio::{fs, sync::RwLock};
use tracing::{error, info, warn};
use uuid::Uuid;

#[inline]
fn jst() -> InvokeResult<FixedOffset> {
    match FixedOffset::east_opt(9 * 60 * 60) {
        Some(jst) => Ok(jst),
        None => {
            error!("failed to create jst fixed offset");
            Err(InvokeError::Chrono)
        }
    }
}

#[tauri::command]
async fn exists_auth(st_admin: State<'_, RwLock<CachedAdmin>>) -> InvokeResult<bool> {
    let path = persistent_file_path()?;
    if !fs::try_exists(&path).await.map_err(print_err)? {
        return Ok(false);
    }

    let json = fs::read_to_string(path).await.map_err(print_err)?;
    let save = serde_json::from_str::<Save>(&json).map_err(print_err)?;
    let json = save.into_text()?;
    let admin = serde_json::from_str::<CachedAdmin>(&json).map_err(print_err)?;

    warn!("do authentication here !!");

    let mut st_admin = st_admin.write().await;
    *st_admin = admin;

    Ok(true)
}

#[tauri::command]
async fn start_auth() -> InvokeResult<()> {
    webbrowser::open(dotenv!("AUTH_URL")).map_err(print_err)?;
    Ok(())
}

#[tauri::command]
async fn obtain_auth(st_admin: State<'_, RwLock<CachedAdmin>>) -> InvokeResult<()> {
    warn!("do authentication here !!");

    // let tkn = String::from("test-token-999");
    // let admin = CachedAdmin { org_id, tkn };
    // let json = serde_json::to_string(&admin).map_err(print_err)?;
    // let save = Save::from_text(json)?;
    // let json = serde_json::to_string(&save).map_err(print_err)?;
    // let path = persistent_file_path()?;
    // fs::write(path, json).await.map_err(print_err)?;

    // let mut st_admin = st_admin.write().await;
    // *st_admin = admin;

    Ok(())
}

#[tauri::command]
async fn load_users(
    st_admin: State<'_, RwLock<CachedAdmin>>,
    st_users: State<'_, RwLock<CachedUsers>>,
) -> InvokeResult<Response> {
    let st_admin = st_admin.read().await;
    let mut st_users = st_users.write().await;

    if st_users.has(&st_admin.org_id) {
        let json = serde_json::to_string(&st_users.users).map_err(print_err)?;
        return Ok(Response::new(json));
    }

    let url = format!(
        "{}/{}/view/?q=users",
        dotenv!("BASE_API_URL"),
        st_admin.org_id
    );
    info!("requesting to {url}");
    let mut users = reqwest::get(url)
        .await
        .map_err(print_err)?
        .json::<Vec<User>>()
        .await
        .map_err(print_err)?;

    let jst = jst()?;
    users
        .iter_mut()
        .for_each(|u| u.created_at = u.created_at.with_timezone(&jst));
    users.sort_unstable_by_key(|u| u.created_at);

    let json = serde_json::to_string(&users).map_err(print_err)?;

    st_users.users = users;
    st_users.org_id = st_admin.org_id.clone();

    Ok(Response::new(json))
}

#[tauri::command]
async fn set_query_user(
    st_query: State<'_, RwLock<CachedQuery>>,
    user: String,
) -> InvokeResult<()> {
    if user.is_empty() {
        warn!("empty input");
        return Err(InvokeError::Input);
    }

    let mut st_query = st_query.write().await;
    st_query.user = user;

    Ok(())
}

#[tauri::command]
async fn set_query_report(
    st_query: State<'_, RwLock<CachedQuery>>,
    report: String,
) -> InvokeResult<()> {
    if report.is_empty() {
        warn!("empty input");
        return Err(InvokeError::Input);
    }

    let mut st_query = st_query.write().await;
    st_query.report = report;

    Ok(())
}

#[tauri::command]
async fn set_query_ym(
    st_query: State<'_, RwLock<CachedQuery>>,
    y: String,
    m: String,
) -> InvokeResult<()> {
    if y.is_empty() || m.is_empty() {
        warn!("empty input");
        return Err(InvokeError::Input);
    }

    let mut st_query = st_query.write().await;
    st_query.y = y;
    st_query.m = m;

    Ok(())
}

#[tauri::command]
async fn load_calls(
    st_admin: State<'_, RwLock<CachedAdmin>>,
    st_query: State<'_, RwLock<CachedQuery>>,
    st_calls: State<'_, RwLock<CachedCalls>>,
) -> InvokeResult<Response> {
    let st_admin = st_admin.read().await;
    let st_query = st_query.read().await;
    let mut st_calls = st_calls.write().await;

    if st_calls.has(&st_query.user, &st_query.y, &st_query.m) {
        let json = serde_json::to_string(&st_calls.calls).map_err(print_err)?;
        return Ok(Response::new(json));
    }

    let user_id = BASE64_STANDARD
        .decode(st_query.user.as_str())
        .map_err(print_err)?;
    let user_id = Uuid::from_slice(&user_id).map_err(print_err)?;

    let url = format!(
        "{}/{}/view?q=calls&y={}&m={}&user={}",
        dotenv!("BASE_API_URL"),
        st_admin.org_id,
        st_query.y,
        st_query.m,
        user_id.to_string()
    );
    info!("requesting to {url}");
    let mut calls = reqwest::get(url)
        .await
        .map_err(print_err)?
        .json::<Calls>()
        .await
        .map_err(print_err)?;

    let jst = jst()?;
    calls
        .morning_calls
        .iter_mut()
        .for_each(|c| c.created_at = c.created_at.with_timezone(&jst));
    calls.morning_calls.sort_unstable_by_key(|c| c.created_at);
    calls
        .evening_calls
        .iter_mut()
        .for_each(|c| c.created_at = c.created_at.with_timezone(&jst));
    calls.evening_calls.sort_unstable_by_key(|c| c.created_at);

    let json = serde_json::to_string(&calls).map_err(print_err)?;

    st_calls.calls = calls;
    st_calls.user = st_query.user.clone();
    st_calls.y = st_query.y.clone();
    st_calls.m = st_query.m.clone();

    Ok(Response::new(json))
}

#[tauri::command]
async fn load_reports(
    st_admin: State<'_, RwLock<CachedAdmin>>,
    st_query: State<'_, RwLock<CachedQuery>>,
    st_reports: State<'_, RwLock<CachedDailyReports>>,
) -> InvokeResult<Response> {
    let st_admin = st_admin.read().await;
    let st_query = st_query.read().await;
    let mut st_reports = st_reports.write().await;

    if st_reports.has(&st_query.user, &st_query.y, &st_query.m) {
        let json = serde_json::to_string(&st_reports.reports).map_err(print_err)?;
        return Ok(Response::new(json));
    }

    let user_id = BASE64_STANDARD.decode(&st_query.user).map_err(print_err)?;
    let user_id = Uuid::from_slice(&user_id).map_err(print_err)?;

    let url = format!(
        "{}/{}/view?q=reports&y={}&m={}&user={}",
        dotenv!("BASE_API_URL"),
        st_admin.org_id,
        st_query.y,
        st_query.m,
        user_id.to_string()
    );
    info!("requesting to {url}");
    let mut reports = reqwest::get(url)
        .await
        .map_err(print_err)?
        .json::<Vec<DailyReportMini>>()
        .await
        .map_err(print_err)?;

    let jst = jst()?;
    reports.iter_mut().for_each(|r| {
        r.created_at = r.created_at.with_timezone(&jst);
        r.updated_at = r.updated_at.with_timezone(&jst);
    });
    reports.sort_unstable_by_key(|r| r.created_at);

    let json = serde_json::to_string(&reports).map_err(print_err)?;

    st_reports.reports = reports;
    st_reports.user = st_query.user.clone();
    st_reports.y = st_query.y.clone();
    st_reports.m = st_query.m.clone();

    Ok(Response::new(json))
}

#[tauri::command]
async fn load_print(
    st_admin: State<'_, RwLock<CachedAdmin>>,
    st_query: State<'_, RwLock<CachedQuery>>,
    st_print: State<'_, RwLock<CachedDailyReportPrint>>,
) -> InvokeResult<Response> {
    let st_admin = st_admin.read().await;
    let st_query = st_query.read().await;
    let mut st_print = st_print.write().await;

    if st_print.has(&st_query.report) {
        let json = serde_json::to_string(&st_print.print).map_err(print_err)?;
        return Ok(Response::new(json));
    }

    let report_id = BASE64_STANDARD
        .decode(&st_query.report)
        .map_err(print_err)?;
    let report_id = Uuid::from_slice(&report_id).map_err(print_err)?;

    let url = format!(
        "{}/{}/view?q=print&report={}",
        dotenv!("BASE_API_URL"),
        st_admin.org_id,
        report_id
    );
    info!("requesting to {url}");
    let mut print = reqwest::get(url)
        .await
        .map_err(print_err)?
        .json::<DailyReportPrint>()
        .await
        .map_err(print_err)?;

    let jst = jst()?;
    print.daily_report.iter_mut().for_each(|r| {
        r.created_at = r.created_at.with_timezone(&jst);
        r.updated_at = r.updated_at.with_timezone(&jst);
    });
    print
        .morning_call
        .iter_mut()
        .for_each(|m| m.created_at = m.created_at.with_timezone(&jst));
    print
        .evening_call
        .iter_mut()
        .for_each(|e| e.created_at = e.created_at.with_timezone(&jst));
    print
        .locations
        .iter_mut()
        .for_each(|l| l.created_at = l.created_at.with_timezone(&jst));
    print.waitings.iter_mut().for_each(|w| {
        w.created_at = w.created_at.with_timezone(&jst);
        w.updated_at = w.updated_at.with_timezone(&jst);
    });
    print.loadings.iter_mut().for_each(|l| {
        l.created_at = l.created_at.with_timezone(&jst);
        l.updated_at = l.updated_at.with_timezone(&jst);
    });
    print.restings.iter_mut().for_each(|r| {
        r.created_at = r.created_at.with_timezone(&jst);
        r.updated_at = r.updated_at.with_timezone(&jst);
    });

    let json = serde_json::to_string(&print).map_err(print_err)?;

    st_print.report = st_query.report.clone();
    st_print.print = print;

    Ok(Response::new(json))
}

#[tauri::command]
async fn load_download(
    st_admin: State<'_, RwLock<CachedAdmin>>,
    st_query: State<'_, RwLock<CachedQuery>>,
    st_photos: State<'_, RwLock<CachedPhotos>>,
) -> InvokeResult<Response> {
    let st_admin = st_admin.read().await;
    let st_query = st_query.read().await;
    let mut st_photos = st_photos.write().await;

    if st_photos.has(&st_query.report) {
        let json = serde_json::to_string(&st_photos.photos).map_err(print_err)?;
        return Ok(Response::new(json));
    }

    let report_id = BASE64_STANDARD
        .decode(&st_query.report)
        .map_err(print_err)?;
    let report_id = Uuid::from_slice(&report_id).map_err(print_err)?;

    let url = format!(
        "{}/{}/view?q=download&report={}",
        dotenv!("BASE_API_URL"),
        st_admin.org_id,
        report_id
    );
    info!("requesting to {url}");
    let mut photos = reqwest::get(url)
        .await
        .map_err(print_err)?
        .json::<Photos>()
        .await
        .map_err(print_err)?;

    if !photos.morning_alc.is_empty() {
        let morning_alc = BASE64_STANDARD
            .decode(&photos.morning_alc)
            .map_err(print_err)?;
        let morning_alc = String::from_utf8(morning_alc).map_err(print_err)?;
        photos.morning_alc = morning_alc;
    }
    if !photos.evening_alc.is_empty() {
        let evening_alc = BASE64_STANDARD
            .decode(&photos.evening_alc)
            .map_err(print_err)?;
        let evening_alc = String::from_utf8(evening_alc).map_err(print_err)?;
        photos.evening_alc = evening_alc;
    }
    if !photos.meter.is_empty() {
        let meter = BASE64_STANDARD.decode(&photos.meter).map_err(print_err)?;
        let meter = String::from_utf8(meter).map_err(print_err)?;
        photos.meter = meter;
    }

    let json = serde_json::to_string(&photos).map_err(print_err)?;

    st_photos.report = st_query.report.clone();
    st_photos.photos = photos;

    Ok(Response::new(json))
}

fn get_code(args: Vec<String>) -> Result<String, String> {
    let Some(arg) = args.get(1) else {
        return Err("unexpected arg".to_string());
    };

    let url = match Url::parse(arg) {
        Ok(url) => {
            if url.scheme() != "nippoviewer" {
                return Err("unexpected schema".to_string());
            }
            if url.host_str() != Some("localhost") {
                return Err("unexpected host".to_string());
            }

            url
        },
        Err(e) => {
            error!("{e}");
            return Err("malformed url".to_string());
        }
    };

    let mut params = url.query_pairs();
    let code = match params.next() {
        Some(kv) => {
            if &kv.0 != "code" {
                return Err("unexpected params".to_string());
            }
            kv.1
        }
        None => return Err("empty params".to_string())
    };

    Ok(code.into())
}

#[inline]
fn emit_auth_error(app: &AppHandle, msg: &str) {
    _ = app.emit_to("main", "auth_failed", msg).map_err(print_err);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, args, _| {
            info!("trying to open new instance with args {args:?}");

            let Some(w) = app.get_webview_window("main") else {
                error!("no main window");
                emit_auth_error(app, "window error");
                return;
            };

            let code = match get_code(args) {
                Ok(s) => s,
                Err(s) => {
                    warn!(s);
                    emit_auth_error(app, "auth error");
                    return
                }
            };

            let st_auth = app.state::<RwLock<CachedAuth>>();
            let mut st_auth = match st_auth.try_write() {
                Ok(l) => l,
                Err(e) => {
                    error!("opening another instance?: {e}");
                    emit_auth_error(app, "instance error");
                    return;
                }
            };
            st_auth.code = code;
            
            _ = app.emit_to("main", "auth_done", ()).map_err(print_err);
            _ = w.set_focus().map_err(print_err);
        }))
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            exists_auth,
            start_auth,
            obtain_auth,
            load_users,
            set_query_user,
            set_query_ym,
            set_query_report,
            load_calls,
            load_reports,
            load_print,
            load_download
        ])
        .setup(|app| {
            app.manage(RwLock::new(CachedAuth::default()));
            app.manage(RwLock::new(CachedAdmin::default()));
            app.manage(RwLock::new(CachedUsers::default()));
            app.manage(RwLock::new(CachedQuery::default()));
            app.manage(RwLock::new(CachedCalls::default()));
            app.manage(RwLock::new(CachedDailyReports::default()));
            app.manage(RwLock::new(CachedDailyReportPrint::default()));
            app.manage(RwLock::new(CachedPhotos::default()));
            _ = app.deep_link().register("nippoviewer").map_err(print_err);

            Ok(())
        })
        .run(tauri::generate_context!())?;
    Ok(())
}
