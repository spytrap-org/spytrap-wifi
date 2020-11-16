use futures::{Sink, SinkExt, Stream, StreamExt};
use crate::errors::*;
use tokio::io::{BufReader, AsyncBufReadExt};

pub async fn stdin<S: Sink<String> + Unpin>(mut sink: S) -> Result<()> {
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    while let Some(line) = reader.next_line().await? {
        trace!("stdin: {:?}", line);
        sink.send(line).await.map_err(|_| anyhow!("sink error"))?;
    }

    Ok(())
}

pub async fn stdout<S: Stream<Item=String> + Unpin>(mut stream: S) -> Result<()> {
    while let Some(line) = stream.next().await {
        println!("{}", line);
    }
    Ok(())
}
