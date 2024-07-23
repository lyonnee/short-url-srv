use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

pub fn init() -> tracing_appender::non_blocking::WorkerGuard{
    // 配置 tracing_subscriber 以 JSON 格式写入日志到文件
    let (file_writer, file_write_guard) =
        tracing_appender::non_blocking(tracing_appender::rolling::daily("./logs", "app.log"));

    let log_file = tracing_subscriber::fmt::layer()
        .json()
        .with_level(true)
        .with_ansi(false)
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .with_writer(file_writer).with_filter(tracing_subscriber::filter::LevelFilter::WARN);

    let log_stdout = tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_ansi(true)
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .with_writer(std::io::stdout).with_filter(tracing_subscriber::filter::LevelFilter::DEBUG);

    tracing_subscriber::registry()
        .with(log_file)
        .with(log_stdout)
        .init();

    return file_write_guard;
}
