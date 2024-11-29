use clap::ValueEnum;
use rocket::fairing::AdHoc;
use rocket::fs::NamedFile;
use rocket::http::Header;
use rocket::tokio::runtime::Runtime;
use rocket::{catch, catchers, get, routes, Config, Request, State};
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

#[derive(ValueEnum, Clone, Debug)]
pub enum ServeMode {
    /// All requests will get the entry file as a response.
    Single,
    /// Try to find the resource in the corresponding path under the root directory, if the resource exists and is a file type, return the resource, otherwise return the entry file.
    Mixed,
    /// Try to find the resource in the corresponding path under the root directory, throw 404 if not found or not a file type.
    Direct,
}

struct ServerState {
    root: PathBuf,
    entry: PathBuf,
    mode: ServeMode,
}

pub struct ServeImpl;

impl ServeImpl {
    pub fn handle(root: PathBuf, entry: PathBuf, port: u16, mode: ServeMode) {
        // validate root
        if !root.is_dir() {
            println!("Invalid root (root is not a directory)");
            return;
        }

        // validate entry
        let entry = root.clone().join(entry);
        if !entry.is_file() {
            println!("Invalid entry (entry is not a file)");
            return;
        }

        let config = Config {
            port,
            address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            ..Config::debug_default() // ..Config::release_default()
        };

        // run on tokio runtime
        Runtime::new().unwrap().block_on(async {
            rocket::build()
                .manage(ServerState { root, entry, mode })
                .configure(config)
                .attach(AdHoc::on_response("CORS", |_, res| {
                    Box::pin(async move {
                        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
                        res.set_header(Header::new("Access-Control-Allow-Methods", "*"));
                        res.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
                        // remove this header to allow iframe
                        res.remove_header("X-Frame-Options");
                    })
                }))
                .mount("/", routes![index])
                .register("/", catchers![not_found, internal_error])
                .launch()
                .await
                .ok()
        });
    }
}

#[get("/<path..>")]
async fn index(state: &State<ServerState>, path: PathBuf) -> Option<NamedFile> {
    match state.mode {
        ServeMode::Single => NamedFile::open(&state.entry).await,
        ServeMode::Mixed => {
            let target = state.root.join(path);
            let target_exists = target.try_exists().unwrap_or(false);

            if target_exists && target.is_file() {
                NamedFile::open(target).await
            } else {
                NamedFile::open(&state.entry).await
            }
        }
        ServeMode::Direct => {
            let target = state.root.join(path);
            NamedFile::open(target).await
        }
    }
    .ok()
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}
