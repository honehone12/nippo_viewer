mod error;
mod save;
mod state;
mod command {
    pub(crate) mod auth;
    pub(crate) mod query;
}

use tauri::{AppHandle, Emitter, Manager, Url};
use tauri_plugin_deep_link::DeepLinkExt;
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use command::{
    auth::*,
    query::*
};
use error::print_err;
use state::*;

fn args_to_code(args: Vec<String>) -> Result<String, String> {
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

            let code = match args_to_code(args) {
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
            obtain_tkn,
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
