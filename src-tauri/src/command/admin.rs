use base64::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use dotenvy_macro::dotenv;
use tauri::State;
use tracing::{info, warn};
use uuid::Uuid;
use crate::{
    error::{print_err, InvokeError, InvokeResult}, 
    state::{CachedUsers, CachedViewer}
};

#[derive(Serialize, Deserialize)]
struct Invited {
    user_email: String
}

#[tauri::command]
pub(crate) async fn invite(
    st_viewer: State<'_, RwLock<CachedViewer>>,
    st_users: State<'_, RwLock<CachedUsers>>,
    user: String
) -> InvokeResult<String> {
    if user.is_empty() {
        warn!("empty input");
        return Err(InvokeError::Input);
    }
    
    let mut st_users = st_users.write().await;
    let st_viewer = st_viewer.read().await;

    if !st_users.users.admin {
        warn!("not admin");
        return Err(InvokeError::Input);
    }

    let Some(idx) = st_users.users.users.iter()
        .position(|u| u.user.id == user && u.invitable) 
    else {
        warn!("invalid input");
        return Err(InvokeError::Input);
    };

    let user_id = BASE64_STANDARD.decode(&user).map_err(print_err)?;
    let user_id = Uuid::from_slice(&user_id).map_err(print_err)?.to_string();

    let http_client = reqwest::Client::new();
    let url = format!(
        "{}/{}/admin",
        dotenv!("BASE_API_URL"),
        st_viewer.org_id,
    );
    let form = vec![
        ("op", "invite"),
        ("user", &user_id)
    ];
    info!("inviting");
    let invited = http_client.post(url)
        .header("Authorization", &format!("Bearer {}", st_viewer.tkn.id_token))
        .form(&form)
        .send().await.map_err(print_err)?
        .json::<Invited>().await.map_err(print_err)?;

    st_users.users.users[idx].invitable = false;

    Ok(invited.user_email)
}

#[derive(Serialize, Deserialize)]
struct Promoted {
    sub_admin: bool
}

#[tauri::command]
pub(crate) async fn promote(
    st_viewer: State<'_, RwLock<CachedViewer>>,
    st_users: State<'_, RwLock<CachedUsers>>,
    user: String
) -> InvokeResult<()> {
    if user.is_empty() {
        warn!("empty input");
        return Err(InvokeError::Input);
    }
    
    let mut st_users = st_users.write().await;
    let st_viewer = st_viewer.read().await;

    if !st_users.users.admin {
        warn!("not admin");
        return Err(InvokeError::Input);
    }
    
    let Some(idx) = st_users.users.users.iter()
        .position(|u| u.user.id == user && u.promotable) 
    else {
        warn!("invalid input");
        return Err(InvokeError::Input);
    };

    let user_id = BASE64_STANDARD.decode(&user).map_err(print_err)?;
    let user_id = Uuid::from_slice(&user_id).map_err(print_err)?.to_string();

    let http_client = reqwest::Client::new();
    let url = format!(
        "{}/{}/admin",
        dotenv!("BASE_API_URL"),
        st_viewer.org_id,
    );
    let form = vec![
        ("op", "promote"),
        ("user", &user_id)
    ];
    info!("promoting");
    let promoted = http_client.post(url)
        .header("Authorization", &format!("Bearer {}", st_viewer.tkn.id_token))
        .form(&form)
        .send().await.map_err(print_err)?
        .json::<Promoted>().await.map_err(print_err)?;

    st_users.users.users[idx].promotable = !promoted.sub_admin;

    Ok(())
}
