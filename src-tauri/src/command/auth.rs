use std::{mem, time::{Duration, SystemTime}};
use dotenvy_macro::dotenv;
use tauri::State;
use tokio::{fs, sync::RwLock};
use tracing::{warn, error};
use crate::{
    error::{print_err, InvokeError, InvokeResult},
    save::{persistent_file_path, Save},
    state::*,
};

#[inline]
fn exp(add: u64) -> InvokeResult<u64> {
    let now = SystemTime::now();
    let Some(exp) = now.checked_add(Duration::from_secs(add)) else {
        error!("unexpected expiration");
        return Err(InvokeError::Arithmetic);
    };
    let exp = exp.duration_since(SystemTime::UNIX_EPOCH).map_err(print_err)?;
    Ok(exp.as_secs())
}

#[tauri::command]
pub(crate) async fn exists_auth(st_viewer: State<'_, RwLock<CachedViewer>>) -> InvokeResult<bool> {
    let path = persistent_file_path()?;
    if !fs::try_exists(&path).await.map_err(print_err)? {
        return Ok(false);
    }

    let json = fs::read_to_string(path).await.map_err(print_err)?;
    let save = serde_json::from_str::<Save>(&json).map_err(print_err)?;
    let json = save.into_text()?;
    let mut viewer = serde_json::from_str::<CachedViewer>(&json).map_err(print_err)?;

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(print_err)?
        .as_secs();

    if now >= viewer.exp {
        let (refresh, exp) = {
            let http_client = reqwest::Client::new();
            let url = format!("{}/oauth2/token", dotenv!("BASE_AUTH_DOMAIN"));
            let form = vec![
                ("grant_type", "refresh_token"),
                ("client_id", dotenv!("CLIENT_ID")),
                ("refresh_token", &viewer.tkn.refresh_token)
            ];
            let refresh = match http_client.post(url)
                .form(&form)
                .send().await.map_err(print_err)
            {
                Ok(res) => res.json::<TokenRefresh>().await.map_err(print_err)?,
                Err(e) => {
                    error!("{e}");
                    warn!("need re-login");
                    return Ok(false);
                }
            };
            let exp = exp(refresh.expires_in)?;

            (refresh, exp)
        };

        viewer.exp = exp;
        viewer.tkn.refresh(refresh);
    }

    let mut st_viewer = st_viewer.write().await;
    *st_viewer = viewer;

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
    st_viewer: State<'_, RwLock<CachedViewer>>
) -> InvokeResult<()> {
    let (org_id, code) = {
        let mut st_auth = st_auth.write().await;
        (mem::take(&mut st_auth.org_id), mem::take(&mut st_auth.code))
    };

    let http_clinet = reqwest::Client::new();
    let url = format!("{}/oauth2/token", dotenv!("BASE_AUTH_DOMAIN"));
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
    let exp = exp(tkn.expires_in)?;

    let viewer = CachedViewer{ 
        org_id, 
        tkn,
        exp
    };
    
    let json = serde_json::to_string(&viewer).map_err(print_err)?;
    let save = Save::from_text(json)?;
    let json = serde_json::to_string(&save).map_err(print_err)?;
    let path = persistent_file_path()?;
    fs::write(path, json).await.map_err(print_err)?;
    
    let mut st_viewer = st_viewer.write().await;
    *st_viewer = viewer;
    
    Ok(())
}
