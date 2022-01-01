//! This module enables watching and live-reloading of Tera templates in
//! debug mode. In release mode, everything should be compiled into the binary
//! and considered immutable.
//!
//! This enables a faster build cycle without losing the benefits of building the
//! service in Rust.
//!
//! This is adapted (and in some cases, lifted from) from the approach Zola uses.

use std::sync::{Arc, RwLock};
use std::{env, thread};

use serde::{Deserialize, Serialize};
use tera::Tera;

#[cfg(feature = "template_watcher")]
use std::{fs::read_dir, path::Path, sync::mpsc::channel, time::Duration};

#[cfg(feature = "template_watcher")]
use notify::{watcher, DebouncedEvent::*, RecursiveMode, Watcher};

/// A `FlashMessage` is a generic message that can be shoved into the Session
/// between requests. This isn't particularly useful for JSON-based workflows, but
/// for the traditional webapp side it works well.
#[derive(Debug, Deserialize, Serialize)]
pub struct FlashMessage {
    pub title: String,
    pub message: String,
}

/// A `TemplateStore` contains a "global" templates reference, along
/// with an optional background thread for monitoring template changes for
/// automatic rebuilding.
#[derive(Debug)]
pub struct TemplateStore {
    pub templates: Arc<RwLock<Tera>>,
    pub watcher: Option<thread::JoinHandle<()>>,
}

/// Loads a glob of Tera templates into memory behind an `Arc<RwLock<>>`. This can be
/// used in `app_data()` calls.
///
/// If the `template_watcher` feature is enabled, then this
/// will watch the glob directory for changes and automatically rebuild the templates as
/// they're updated.
pub fn load() -> TemplateStore {
    let templates_glob = env::var("TEMPLATES_GLOB").expect("TEMPLATES_GLOB not set!");
    let templates = Arc::new(RwLock::new(
        Tera::new(&templates_glob).expect("Unable to compile templates!"),
    ));

    #[cfg(feature = "template_watcher")]
    let store = templates.clone();

    #[cfg(feature = "template_watcher")]
    let watcher = Some(thread::spawn(move || {
        let (tx, rx) = channel();
        let mut watcher =
            watcher(tx, Duration::from_secs(1)).expect("Template watcher creation failed!");

        let path = templates_glob.replace("**/*", "");
        let watcher_err_msg = format!("Can't watch for changes in folder `{}`. Does it exist, and do you have correct permissions?", path);
        watcher
            .watch(path, RecursiveMode::Recursive)
            .expect(&watcher_err_msg);

        loop {
            match rx.recv() {
                Ok(event) => {
                    match event {
                        // Intellij does weird things on edit, chmod is there to count those changes
                        // https://github.com/passcod/notify/issues/150#issuecomment-494912080
                        Rename(_, path)
                        | Create(path)
                        | Write(path)
                        | Remove(path)
                        | Chmod(path) => {
                            if is_temp_file(&path) {
                                continue;
                            }

                            // We only care about changes in non-empty folders
                            if path.is_dir() && is_folder_empty(&path) {
                                continue;
                            }

                            info!("Change detected @ {}", path.display());

                            let mut lock = store
                                .write()
                                .expect("Unable to acquire write lock on Templates!");
                            if let Err(e) = lock.full_reload() {
                                error!("Unable to reload Templates! {:?}", e);
                            }
                        }

                        // Theoretically unreachable, for our purposes.
                        // Perhaps mark with unreachable? Meh, this is debug code.
                        _ => {}
                    }
                }

                Err(e) => {
                    error!("Error in template reloading: {:?}", e);
                }
            }
        }
    }));

    #[cfg(not(feature = "template_watcher"))]
    let watcher = None;

    TemplateStore {
        templates: templates,
        watcher: watcher,
    }
}

/// Returns whether the path we received corresponds to a temp file created
/// by an editor or the OS
#[cfg(feature = "template_watcher")]
fn is_temp_file(path: &Path) -> bool {
    let ext = path.extension();

    match ext {
        Some(ex) => match ex.to_str().unwrap() {
            "swp" | "swx" | "tmp" | ".DS_STORE" => true,

            // jetbrains IDE
            x if x.ends_with("jb_old___") => true,
            x if x.ends_with("jb_tmp___") => true,
            x if x.ends_with("jb_bak___") => true,

            // vim
            x if x.ends_with('~') => true,

            _ => {
                if let Some(filename) = path.file_stem() {
                    // emacs
                    let name = filename.to_str().unwrap();
                    name.starts_with('#') || name.starts_with(".#")
                } else {
                    false
                }
            }
        },

        None => true,
    }
}

/// Check if the directory at path contains any file
#[cfg(feature = "template_watcher")]
fn is_folder_empty(dir: &Path) -> bool {
    // Can panic if we don't have the rights I guess?
    let files: Vec<_> = read_dir(dir)
        .expect("Failed to read a directory to see if it was empty")
        .collect();

    files.is_empty()
}
