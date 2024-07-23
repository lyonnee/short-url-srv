pub fn init() {
    // 配置 tracing_subscriber 以 JSON 格式写入日志到文件
    let file_appender = tracing_appender::rolling::minutely("./logs", "app.log");
    let (file_writer, _) = tracing_appender::non_blocking(file_appender);

    // let (console_writer, _) = tracing_appender::non_blocking(std::io::stdout());

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_writer(file_writer)
        .with_writer(std::io::stdout)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
