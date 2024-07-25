use std::str::FromStr;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

use crate::infra::config;

pub fn init() -> Option<tracing_appender::non_blocking::WorkerGuard> {
    let lock_config = config::get_configs();
    let config = lock_config.as_ref().unwrap();

    let mut log_stdout= None;
    if config.log.stdout.enable == true {
        let level = tracing_subscriber::filter::LevelFilter::from_str(&config.log.stdout.level)
            .unwrap_or(tracing_subscriber::filter::LevelFilter::INFO);

        log_stdout = Some( tracing_subscriber::fmt::layer()
            .with_level(true)
            .with_ansi(true)
            .with_target(false)
            .with_file(true)
            .with_line_number(true)
            .with_writer(std::io::stdout)
            .with_filter(level))
    }

    let mut log_file= None;
    let file_write_guard = if config.log.file.enable == true {
        // 配置 tracing_subscriber 以 JSON 格式写入日志到文件
        let (file_writer, file_write_guard) =
            tracing_appender::non_blocking(tracing_appender::rolling::daily(
                &config.log.file.filepath,
                &config.log.file.filename,
            ));

        let level = tracing_subscriber::filter::LevelFilter::from_str(&config.log.file.level)
            .unwrap_or(tracing_subscriber::filter::LevelFilter::INFO);

        log_file = Some(tracing_subscriber::fmt::layer()
            .json()
            .with_level(true)
            .with_ansi(false)
            .with_target(false)
            .with_file(true)
            .with_line_number(true)
            .with_writer(file_writer)
            .with_filter(level));

        Some(file_write_guard)
    } else {
        None
    };

    tracing_subscriber::registry().with(log_stdout).with(log_file).init();

    tracing::info!("The logger component has been initialized!!!");

    return file_write_guard;
}
