
mod character;
mod index;

use std::ffi::OsStr;
use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use actix_files::Files;
use actix_web::rt::time::interval;
use actix_web::web::Data;
use actix_web::{get, web, Responder};
use actix_web_lab::{
    sse::{self, Sse},
    util::InfallibleStream,
};
use futures_util::future;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use actix_multipart::form::tempfile::TempFile;
use tera::Tera;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tracing::{error, info, warn};

pub fn scope_config(cfg: &mut web::ServiceConfig) {
    let tera = Tera::new("res/html/**/*").unwrap();

    cfg.app_data(Data::new(tera))
        .service(event_stream)
        .service(Files::new("/assets", "res/assets"))
        .service(Files::new("/css", "res/css"))
        .service(Files::new("/js", "res/js"));
}

#[derive(Serialize, Deserialize)]
pub enum EventMsgLevel {
    Info,
    Warn,
    Error,
}

#[derive(Serialize)]
pub struct EventMsg {
    pub level: EventMsgLevel,
    pub msg: String,
}

pub struct Broadcaster {
    inner: Mutex<BroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
pub struct BroadcasterInner {
    clients: Vec<mpsc::Sender<sse::Event>>,
}
impl Broadcaster {
    /// Constructs new broadcaster and spawns ping loop.
    pub fn create() -> Arc<Self> {
        let this = Arc::new(Broadcaster {
            inner: Mutex::new(BroadcasterInner::default()),
        });

        Broadcaster::spawn_ping(Arc::clone(&this));

        this
    }

    /// Pings clients every 10 seconds to see if they are alive and remove them from the broadcast
    /// list if not.
    fn spawn_ping(this: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    /// Removes all non-responsive clients from broadcast list.
    async fn remove_stale_clients(&self) {
        let clients = self.inner.lock().clients.clone();

        let mut ok_clients = Vec::new();

        for client in clients {
            if client.send(sse::Event::Comment("ping".into())).await.is_ok() {
                ok_clients.push(client.clone());
            }
        }

        self.inner.lock().clients = ok_clients;
    }

    /// Registers client with broadcaster, returning an SSE response body.
    pub async fn new_client(&self) -> Sse<InfallibleStream<ReceiverStream<sse::Event>>> {
        let (tx, rx) = mpsc::channel(10);

        tx.send(sse::Data::new("connected").into()).await.unwrap();

        self.inner.lock().clients.push(tx);

        Sse::from_infallible_receiver(rx)
    }

    /// Broadcasts `msg` to all clients.
    pub async fn broadcast(&self, msg: EventMsg) {
        let clients = self.inner.lock().clients.clone();

        if let Ok(msg) = sse::Data::new_json(msg) {
            let send_futures = clients.iter().map(|client| client.send(msg.clone().into()));

            // try to send to all clients, ignoring failures
            // disconnected clients will get swept up by `remove_stale_clients`
            let _ = future::join_all(send_futures).await;
        }
    }

    pub async fn info(&self, msg: &str) {
        let msg = EventMsg {
            level: EventMsgLevel::Info,
            msg: msg.to_string(),
        };
        self.broadcast(msg).await;
    }

    pub async fn warn(&self, msg: &str) {
        let msg = EventMsg {
            level: EventMsgLevel::Warn,
            msg: msg.to_string(),
        };
        self.broadcast(msg).await;
    }

    pub async fn error(&self, msg: &str) {
        let msg = EventMsg {
            level: EventMsgLevel::Error,
            msg: msg.to_string(),
        };
        self.broadcast(msg).await;
    }
}

#[get("/events")]
async fn event_stream(broadcaster: Data<Broadcaster>) -> impl Responder {
    broadcaster.new_client().await
}


macro_rules! redirect {
    ($url: expr) => {
        HttpResponse::Found().append_header(("Location", $url)).finish()
    };
}


// pub fn save_file(target_dir: &PathBuf, temp_file: Option<TempFile>, file_name: &str) -> Result<(String, String), ()> {
//     let md5sum;
//
//     if let Some(f) = temp_file {
//         if !target_dir.exists() {
//             if let Err(e) = fs::create_dir_all(&target_dir) {
//                 error!("Failed to create file folder {:?}: {}", &target_dir.to_str(), e);
//                 return Err(());
//             }
//         }
//
//         let mut hasher = Md5::new();
//         let temp_file_name = f.file_name.unwrap_or_default();
//         let extension = Path::new(temp_file_name.as_str())
//             .extension()
//             .unwrap_or(OsStr::new("png"))
//             .to_str()
//             .unwrap_or("png");
//         let open_f = f.file.reopen().map_err(|_| {
//             error!("Failed to reopen temp file");
//             ()
//         })?;
//         let mut reader = BufReader::new(open_f);
//         let mut buf = Vec::new();
//         reader.read_to_end(&mut buf).map_err(|_| {
//             error!("Failed to read temp file");
//             ()
//         })?;
//         hasher.update(&buf);
//         md5sum = format!("{:x}", hasher.finalize());
//
//         let saved_name = if file_name.is_empty() {
//             format!("{}.{}", md5sum, extension)
//         } else {
//             String::from(file_name)
//         };
//
//         let path = target_dir.join(saved_name.as_str());
//         if !path.exists() {
//             if let Err(e) = f.file.persist(&path) {
//                 warn!("Failed to save file to {:?}: {}", &path.to_str(), e);
//                 warn!("Try copying");
//                 if let Err(e) = fs::copy(e.file.path(), &path) {
//                     error!("Failed to copy file: {}", e);
//                     return Err(());
//                 }
//             }
//         } else {
//             info!("File {saved_name} exists");
//         }
//
//         return Ok((md5sum, saved_name));
//     }
//     Ok((String::new(), String::new()))
// }
//
// pub fn delete_file(root_dir: &PathBuf, file_path: String) {
//     if !file_path.is_empty() {
//         let real_path = root_dir.join(&file_path);
//         if let Err(e) = fs::remove_file(&real_path) {
//             error!("Failed to remove file {}: {}", file_path, e);
//         }
//     }
// }
