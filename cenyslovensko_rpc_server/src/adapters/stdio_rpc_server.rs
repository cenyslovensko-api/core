use anyhow::Result;
use tokio::io::{self, AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};

use crate::domain::{RpcRequest, RpcResponse};
use crate::ports::RpcRequestHandler;

pub async fn run<THandler>(handler: &THandler) -> Result<()>
where
    THandler: RpcRequestHandler,
{
    run_with_io(handler, io::stdin(), io::stdout()).await
}

pub async fn run_with_io<THandler, TInput, TOutput>(
    handler: &THandler,
    input: TInput,
    mut output: TOutput,
) -> Result<()>
where
    THandler: RpcRequestHandler,
    TInput: AsyncRead + Unpin,
    TOutput: AsyncWrite + Unpin,
{
    let mut lines = BufReader::new(input).lines();

    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }

        let response = match serde_json::from_str::<RpcRequest>(&line) {
            Ok(request) => handler.handle_request(request).await,
            Err(_) => RpcResponse::parse_error(),
        };

        let serialized = serde_json::to_string(&response)?;
        output.write_all(serialized.as_bytes()).await?;
        output.write_all(b"\n").await?;
        output.flush().await?;
    }

    Ok(())
}
