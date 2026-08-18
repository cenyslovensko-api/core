use anyhow::Result;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::domain::{RpcRequest, RpcResponse};
use crate::ports::RpcRequestHandler;

pub async fn run<THandler>(handler: &THandler) -> Result<()>
where
    THandler: RpcRequestHandler,
{
    let mut lines = BufReader::new(io::stdin()).lines();
    let mut stdout = io::stdout();

    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }

        let response = match serde_json::from_str::<RpcRequest>(&line) {
            Ok(request) => handler.handle_request(request).await,
            Err(_) => RpcResponse::parse_error(),
        };

        let serialized = serde_json::to_string(&response)?;
        stdout.write_all(serialized.as_bytes()).await?;
        stdout.write_all(b"\n").await?;
        stdout.flush().await?;
    }

    Ok(())
}
