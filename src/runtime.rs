use crate::error::Error;

pub(crate) struct Runtime {
    tokio: tokio::runtime::Runtime,
    s3_client: aws_sdk_s3::Client,
}

impl Runtime {
    pub(crate) fn new() -> Result<Runtime, Error> {
        let tokio = new_tokio_runtime()?;
        let s3_client = new_s3_client(&tokio)?;
        Ok(Runtime { tokio, s3_client })
    }
    pub(crate) fn tokio(&self) -> &tokio::runtime::Runtime { &self.tokio }
    pub(crate) fn s3_client(&self) -> &aws_sdk_s3::Client { &self.s3_client }
}

fn new_tokio_runtime() -> Result<tokio::runtime::Runtime, Error> {
    let tokio =
        tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()?;
    Ok(tokio)
}

fn new_s3_client(tokio: &tokio::runtime::Runtime) -> Result<aws_sdk_s3::Client, Error> {
    tokio.block_on(async {
        let s3_config = aws_config::load_from_env().await;
        let s3_client = aws_sdk_s3::Client::new(&s3_config);
        Ok(s3_client)
    })
}