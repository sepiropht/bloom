use env_logger::Builder;
use kernel::{Error, config::Config, drivers::{mailer::ses::SesMailer, queue::postgres::PostgresQueue, storage::s3::S3Storage}};
use std::sync::Arc;

pub fn run() -> Result<(), Error> {
    let config = Config::load()?;
    let log_level = if config.debug {
        stdx::log::LevelFilter::Debug
    } else {
        stdx::log::LevelFilter::Info
    };
    Builder::new().filter_level(log_level).init();

    let mut runtime = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async move {
        let db = kernel::db::connect(&config.database).await?;
        let queue = Arc::new(PostgresQueue::new(db.clone()));
        let mailer = Arc::new(SesMailer::new());
        let storage = Arc::new(S3Storage::new());

        let kernel_service = Arc::new(kernel::Service::new(config, db, queue.clone(), mailer, storage));

        worker::run(kernel_service, queue).await
    })
}
