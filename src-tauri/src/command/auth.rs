use dotenvy_macro::dotenv;
use tauri::State;
use tokio::{fs, sync::RwLock};
use tracing::warn;
use crate::{
    error::{print_err, InvokeResult},
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
pub(crate) async fn start_auth() -> InvokeResult<()> {
    webbrowser::open(dotenv!("AUTH_URL")).map_err(print_err)?;
    Ok(())
}

#[tauri::command]
pub(crate) async fn obtain_tkn(
    st_auth: State<'_, RwLock<CachedAuth>>,
    st_admin: State<'_, RwLock<CachedAdmin>>
) -> InvokeResult<()> {
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
