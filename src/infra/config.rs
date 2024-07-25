
use std::{path::Path, sync::{mpsc::channel, Arc, RwLock, RwLockReadGuard}, time::Duration};

use serde::Deserialize;
use confique::Config;
use lazy_static::lazy_static;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};

#[derive(Config, Debug, Deserialize)]
#[allow(unused)]
pub struct Configs {
    pub http: Http,
    pub log: Log,
    pub mysql: Mysql,
}

#[derive(Config, Debug, Deserialize)]
#[allow(unused)]
pub struct Http {
    #[config(default = "0.0.0.0:10240")]
    pub addr: String,
}

#[derive(Config, Debug, Deserialize)]
#[allow(unused)]
pub struct Log {
    #[config(default = "info")]
    pub level: String,
    #[config(default = "logs/")]
    pub filepath: String,
    #[config(default = "app.log")]
    pub filename: String,
}

#[derive(Config, Debug, Deserialize)]
#[allow(unused)]
pub struct Mysql {
    pub dsn: String,
    #[config(default = 256)]
    pub max_conns: u32,
}

lazy_static! {
    static ref CONFIGS: Arc<RwLock<Option<Configs>>> = Arc::new(RwLock::new(None));
}

pub fn get_configs() -> RwLockReadGuard<'static, Option<Configs>> {
    let configs_guard = CONFIGS.read().unwrap();
    configs_guard
}


pub fn init(env: String) -> Result<(), confique::Error> {
    let filepath = env + ".yaml";
    let cfg: Configs = Configs::builder().file(&filepath).load()?;

    let mut configs_lock = CONFIGS.write().unwrap();
    *configs_lock = Some(cfg);

    tokio::spawn(async move{
        watch(&filepath)
    });

    tracing::info!("The config has been loaded!!!");

    Ok(())
}

pub fn watch(filepath: &str) {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        notify::Config::default().with_poll_interval(Duration::from_secs(2)),
    )
    .unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(Path::new(filepath), RecursiveMode::NonRecursive)
        .unwrap();

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(Ok(Event {
                kind: notify::event::EventKind::Modify(_),
                ..
            })) => {
                println!(" * config file written; refreshing configuration ...");
                let res = Configs::builder().file(filepath).load();

                match res {
                    Ok(cfg) => {
                        let mut configs_lock = CONFIGS.write().unwrap();
                        *configs_lock = Some(cfg);
                    },
                    Err(e) => {
                        println!("watch failed: {:?}", e)
                    },
                }
            }

            Err(e) => println!("watch error: {:?}", e),

            _ => {
                // Ignore event
            }
        }
    }
}