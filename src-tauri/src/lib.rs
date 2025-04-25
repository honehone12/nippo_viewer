mod error;
mod save;
mod state;
mod command {
    pub(crate) mod auth;
    pub(crate) mod query;
}

use std::{borrow::Cow, collections::HashMap};

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_deep_link::{DeepLinkExt, OpenUrlEvent};
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use command::{
    auth::*,
    query::*
};
use error::print_err;
use state::*;

fn handle_link_event(event: OpenUrlEvent) -> Result<String, String> {
    let mut urls = event.urls();
    if urls.len() != 1 {
        return Err("unexpected event".to_string());
    }
    let url = urls.remove(0);

    if url.scheme() != "nippoviewer" {
        return Err("unexpected schema".to_string());
    }
    if url.host_str() != Some("localhost") {
        return Err("unexpected host".to_string());
    }

    let params = url.query_pairs().collect::<HashMap<Cow<'_, str>, Cow<'_, str>>>();
    let Some(code) = params.get("code") else {
        return Err("unexpected params".to_string());
    };

    Ok(code.to_string())
}

#[inline]
fn emit_auth_error(app: &AppHandle, msg: &str) {
    _ = app.emit_to("main", "auth_failed", msg).map_err(print_err);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let builder = tauri::Builder::default();

    #[cfg(desktop)]
    let builder = builder.plugin(tauri_plugin_single_instance::init(|app, args, _| {
        info!("trying to open new instance with args {args:?}");

        let Some(w) = app.get_webview_window("main") else {
            error!("no main window");
            return;
        };
        _ = w.set_focus().map_err(print_err);
    }));
    
    builder.plugin(tauri_plugin_deep_link::init())
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
        let handle = app.handle();

        {
            let app_handle = handle.clone();
            app.deep_link().on_open_url(move |e| {
                let code = match handle_link_event(e) {
                    Ok(s) => s,
                    Err(s) => {
                        warn!(s);
                        emit_auth_error(&app_handle, "auth error");
                        return
                    }
                };
        
                let st_auth = app_handle.state::<RwLock<CachedAuth>>();
                let mut st_auth = match st_auth.try_write() {
                    Ok(l) => l,
                    Err(e) => {
                        error!("opening another instance?: {e}");
                        emit_auth_error(&app_handle, "instance error");
                        return;
                    }
                };
                st_auth.code = code;
                
                _ = app_handle.emit_to("main", "auth_done", ()).map_err(print_err);
            });
        }

        Ok(())
    })
    .run(tauri::generate_context!())?;

    Ok(())
}
