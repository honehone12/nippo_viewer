use base64::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use dotenvy_macro::dotenv;
use tauri::{State, ipc::Response};
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
) -> InvokeResult<Response> {
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
    let Some(idx) =st_users.users.invitables.iter().position(|u| u.id == user) else {
        warn!("invalid input");
        return Err(InvokeError::Input);
    };

    let user_id = BASE64_STANDARD.decode(&user).map_err(print_err)?;
    let user_id = Uuid::from_slice(&user_id).map_err(print_err)?;

    let http_client = reqwest::Client::new();
    let url = format!(
        "{}/{}/admin?op=invite&user={}",
        dotenv!("BASE_API_URL"),
        st_viewer.org_id,
        user_id
    );
    info!("inviting");
    let invited = http_client.post(url)
        .header("Authorization", &format!("Bearer {}", st_viewer.tkn.id_token))
        .send().await.map_err(print_err)?
        .json::<Invited>().await.map_err(print_err)?;

    st_users.users.invitables.remove(idx);

    Ok(Response::new(invited.user_email))
}