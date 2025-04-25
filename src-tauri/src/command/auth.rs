use std::mem;
use dotenvy_macro::dotenv;
use serde_json::Value;
use tauri::State;
use tokio::{fs, sync::RwLock};
use tracing::{info, warn};
use crate::{
    error::{print_err, InvokeError, InvokeResult},
    save::{persistent_file_path, Save},
    state::*,
};

#[tauri::command]
pub(crate) async fn exists_auth(st_admin: State<'_, RwLock<CachedAdmin>>) -> InvokeResult<bool> {
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
pub(crate) async fn start_auth(
    org_id: String, 
    st_auth: State<'_, RwLock<CachedAuth>>
) -> InvokeResult<()> {
    if org_id.is_empty() {
        warn!("emprty input");
        return Err(InvokeError::Input);
    }

    let mut st_auth = st_auth.write().await;
    st_auth.org_id = org_id; 

    webbrowser::open(dotenv!("AUTH_URL")).map_err(print_err)?;
    Ok(())
}

#[tauri::command]
pub(crate) async fn obtain_tkn(
    st_auth: State<'_, RwLock<CachedAuth>>,
    st_admin: State<'_, RwLock<CachedAdmin>>
) -> InvokeResult<()> {
    let (org_id, code) = {
        let mut st_auth = st_auth.write().await;
        (mem::take(&mut st_auth.org_id), mem::take(&mut st_auth.code))
    };

    let http_clinet = reqwest::Client::new();
    let url = format!(
        "{}/oauth2/token",
        dotenv!("BASE_AUTH_DOMAIN"),
    );
    let form = vec![
        ("grant_type", "authorization_code"),
        ("client_id", dotenv!("CLIENT_ID")),
        ("code", &code),
        ("redirect_uri", dotenv!("REDIRECT_URI"))
    ];
    let tkn = http_clinet.post(url)
        .form(&form)
        .send().await.map_err(print_err)?
        .json::<Token>().await.map_err(print_err)?;

    let admin = CachedAdmin{ 
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
