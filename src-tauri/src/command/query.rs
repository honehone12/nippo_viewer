use base64::prelude::*;
use chrono::FixedOffset;
use dotenvy_macro::dotenv;
use tauri::{ipc::Response, State};
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use uuid::Uuid;
use crate::{
    error::{print_err, InvokeError, InvokeResult},
    state::*
};

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
pub(crate) async fn set_query_user(
    st_users: State<'_, RwLock<CachedUsers>>,
    st_query: State<'_, RwLock<CachedQuery>>,
    user: String,
) -> InvokeResult<()> {
    if user.is_empty() {
        warn!("empty input");
        return Err(InvokeError::Input);
    }

    let st_users = st_users.read().await;

    if st_users.users.users.iter().position(|u| u.user.id == user).is_none() {
        warn!("invalid input");
        return Err(InvokeError::Input);
    }

    let mut st_query = st_query.write().await;
    st_query.user = user;

    Ok(())
}

#[tauri::command]
pub(crate) async fn set_query_report(
    st_query: State<'_, RwLock<CachedQuery>>,
    st_reports: State<'_, RwLock<CachedDailyReports>>,
    report: String,
) -> InvokeResult<()> {
    if report.is_empty() {
        warn!("empty input");
        return Err(InvokeError::Input);
    }

    let st_reports = st_reports.read().await;

    if st_reports.reports.iter().position(|r| r.id == report).is_none() {
        warn!("invalid input");
        return Err(InvokeError::Input);
    }

    let mut st_query = st_query.write().await;
    st_query.report = report;

    Ok(())
}

#[tauri::command]
pub(crate) async fn set_query_ym(
    st_query: State<'_, RwLock<CachedQuery>>,
    y: String,
    m: String,
) -> InvokeResult<()> {
    if y.is_empty() || y.len() != 4 || m.is_empty() || m.len() > 2 {
        warn!("empty input");
        return Err(InvokeError::Input);
    }

    let mut st_query = st_query.write().await;
    st_query.y = y;
    st_query.m = m;

    Ok(())
}

#[tauri::command]
pub(crate) async fn load_users(
    st_viewer: State<'_, RwLock<CachedViewer>>,
    st_users: State<'_, RwLock<CachedUsers>>,
) -> InvokeResult<Response> {
    let st_viewer = st_viewer.read().await;
    let mut st_users = st_users.write().await;

    if st_users.has(&st_viewer.org_id) {
        let json = serde_json::to_string(&st_users.users).map_err(print_err)?;
        return Ok(Response::new(json));
    }

    let http_client = reqwest::Client::new();
    let url = format!(
        "{}/{}/view/?q=users",
        dotenv!("BASE_API_URL"),
        st_viewer.org_id
    );
    info!("requesting users");
    let mut users = http_client.get(url)
        .header("Authorization", &format!("Bearer {}", st_viewer.tkn.id_token))
        .send().await.map_err(print_err)?
        .json::<Users>().await.map_err(print_err)?;

    let jst = jst()?;
    users.users.iter_mut()
        .for_each(|u| u.user.created_at = u.user.created_at.with_timezone(&jst));
    users.users.sort_unstable_by_key(|u| u.user.created_at);

    let json = serde_json::to_string(&users).map_err(print_err)?;
    st_users.users = users;
    st_users.org_id = st_viewer.org_id;

    Ok(Response::new(json))
}

#[tauri::command]
pub(crate) async fn load_calls(
    st_viewer: State<'_, RwLock<CachedViewer>>,
    st_query: State<'_, RwLock<CachedQuery>>,
    st_calls: State<'_, RwLock<CachedCalls>>,
) -> InvokeResult<Response> {
    let st_viewer = st_viewer.read().await;
    let st_query = st_query.read().await;
    let mut st_calls = st_calls.write().await;

    if st_calls.has(&st_query.user, &st_query.y, &st_query.m) {
        let json = serde_json::to_string(&st_calls.calls).map_err(print_err)?;
        return Ok(Response::new(json));
    }

    let user_id = BASE64_STANDARD.decode(&st_query.user).map_err(print_err)?;
    let user_id = Uuid::from_slice(&user_id).map_err(print_err)?;

    let http_client = reqwest::Client::new();
    let url = format!(
        "{}/{}/view?q=calls&y={}&m={}&user={}",
        dotenv!("BASE_API_URL"),
        st_viewer.org_id,
        st_query.y,
        st_query.m,
        user_id
    );
    info!("requesting calls");
    let mut calls = http_client.get(url)
        .header("Authorization", &format!("Bearer {}", st_viewer.tkn.id_token))
        .send().await.map_err(print_err)?
        .json::<Calls>().await.map_err(print_err)?;

    let jst = jst()?;
    calls.morning_calls.iter_mut().for_each(|c| {
        c.created_at = c.created_at.with_timezone(&jst);
        c.license_expiration = c.license_expiration.with_timezone(&jst);
    });
    calls.morning_calls.sort_unstable_by_key(|c| c.created_at);
    calls.evening_calls.iter_mut()
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
pub(crate) async fn load_reports(
    st_viewer: State<'_, RwLock<CachedViewer>>,
    st_query: State<'_, RwLock<CachedQuery>>,
    st_reports: State<'_, RwLock<CachedDailyReports>>,
) -> InvokeResult<Response> {
    let st_viewer = st_viewer.read().await;
    let st_query = st_query.read().await;
    let mut st_reports = st_reports.write().await;

    if st_reports.has(&st_query.user, &st_query.y, &st_query.m) {
        let json = serde_json::to_string(&st_reports.reports).map_err(print_err)?;
        return Ok(Response::new(json));
    }

    let user_id = BASE64_STANDARD.decode(&st_query.user).map_err(print_err)?;
    let user_id = Uuid::from_slice(&user_id).map_err(print_err)?;

    let http_client = reqwest::Client::new();
    let url = format!(
        "{}/{}/view?q=reports&y={}&m={}&user={}",
        dotenv!("BASE_API_URL"),
        st_viewer.org_id,
        st_query.y,
        st_query.m,
        user_id
    );
    info!("requesting reports");
    let mut reports = http_client.get(url)
        .header("Authorization", &format!("Bearer {}", st_viewer.tkn.id_token))
        .send().await.map_err(print_err)?
        .json::<Vec<DailyReportMini>>().await.map_err(print_err)?;

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
pub(crate) async fn load_print(
    st_viewer: State<'_, RwLock<CachedViewer>>,
    st_query: State<'_, RwLock<CachedQuery>>,
    st_print: State<'_, RwLock<CachedDailyReportPrint>>,
) -> InvokeResult<Response> {
    let st_viewer = st_viewer.read().await;
    let st_query = st_query.read().await;
    let mut st_print = st_print.write().await;

    if st_print.has(&st_query.report) {
        let json = serde_json::to_string(&st_print.print).map_err(print_err)?;
        return Ok(Response::new(json));
    }

    let report_id = BASE64_STANDARD.decode(&st_query.report).map_err(print_err)?;
    let report_id = Uuid::from_slice(&report_id).map_err(print_err)?;

    let http_client = reqwest::Client::new();
    let url = format!(
        "{}/{}/view?q=print&report={}",
        dotenv!("BASE_API_URL"),
        st_viewer.org_id,
        report_id
    );
    info!("requesting print");
    let mut print = http_client.get(url)
        .header("Authorization", &format!("Bearer {}", st_viewer.tkn.id_token))
        .send().await.map_err(print_err)?
        .json::<DailyReportPrint>().await.map_err(print_err)?;

    let jst = jst()?;
    print.daily_report.iter_mut().for_each(|r| {
        r.created_at = r.created_at.with_timezone(&jst);
        r.updated_at = r.updated_at.with_timezone(&jst);
    });
    print.morning_call.iter_mut()
        .for_each(|m| m.created_at = m.created_at.with_timezone(&jst));
    print.evening_call.iter_mut()
        .for_each(|e| e.created_at = e.created_at.with_timezone(&jst));
    print.sites.iter_mut()
        .for_each(|s| s.created_at = s.created_at.with_timezone(&jst));
    print.locations.iter_mut()
        .for_each(|l| l.created_at = l.created_at.with_timezone(&jst));
    print.waitings.iter_mut().for_each(|w| {
        w.created_at = w.created_at.with_timezone(&jst);
        w.updated_at = w.updated_at.with_timezone(&jst);
    });
    print.loadings.iter_mut().for_each(|l| {
        l.created_at = l.created_at.with_timezone(&jst);
        l.updated_at = l.updated_at.with_timezone(&jst);
    });
    print.restings.iter_mut()
        .for_each(|r| r.created_at = r.created_at.with_timezone(&jst));

    let json = serde_json::to_string(&print).map_err(print_err)?;

    st_print.report = st_query.report.clone();
    st_print.print = print;

    Ok(Response::new(json))
}

#[tauri::command] 
pub(crate) async fn load_download(
    st_viewer: State<'_, RwLock<CachedViewer>>,
    st_query: State<'_, RwLock<CachedQuery>>,
    st_photos: State<'_, RwLock<CachedPhotos>>,
) -> InvokeResult<Response> {
    let st_viewer = st_viewer.read().await;
    let st_query = st_query.read().await;
    let mut st_photos = st_photos.write().await;

    if st_photos.has(&st_query.report) {
        let json = serde_json::to_string(&st_photos.photos).map_err(print_err)?;
        return Ok(Response::new(json));
    }

    let report_id = BASE64_STANDARD.decode(&st_query.report).map_err(print_err)?;
    let report_id = Uuid::from_slice(&report_id).map_err(print_err)?;

    let http_client = reqwest::Client::new();
    let url = format!(
        "{}/{}/view?q=download&report={}",
        dotenv!("BASE_API_URL"),
        st_viewer.org_id,
        report_id
    );
    info!("requesting photos");
    let mut photos = http_client.get(url)
        .header("Authorization", &format!("Bearer {}", st_viewer.tkn.id_token))
        .send().await.map_err(print_err)?
        .json::<Photos>().await.map_err(print_err)?;

    if !photos.morning_alc.is_empty() {
        let morning_alc = BASE64_STANDARD
            .decode(&photos.morning_alc).map_err(print_err)?;
        let morning_alc = String::from_utf8(morning_alc).map_err(print_err)?;
        photos.morning_alc = morning_alc;
    }
    if !photos.evening_alc.is_empty() {
        let evening_alc = BASE64_STANDARD
            .decode(&photos.evening_alc).map_err(print_err)?;
        let evening_alc = String::from_utf8(evening_alc).map_err(print_err)?;
        photos.evening_alc = evening_alc;
    }
    if !photos.morning_mtr.is_empty() {
        let morning_mtr = BASE64_STANDARD.decode(&photos.morning_mtr).map_err(print_err)?;
        let morning_mtr = String::from_utf8(morning_mtr).map_err(print_err)?;
        photos.morning_mtr = morning_mtr;
    }
    if !photos.evening_mtr.is_empty() {
        let evening_mtr = BASE64_STANDARD.decode(&photos.evening_mtr).map_err(print_err)?;
        let evening_mtr = String::from_utf8(evening_mtr).map_err(print_err)?;
        photos.evening_mtr = evening_mtr;
    }

    let json = serde_json::to_string(&photos).map_err(print_err)?;

    st_photos.report = st_query.report.clone();
    st_photos.photos = photos;

    Ok(Response::new(json))
}
