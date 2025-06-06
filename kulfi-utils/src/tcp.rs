/// this is the tcp proxy.
///
/// the other side has indicated they want to access our TCP device, whose id is specified in the
/// protocol header. we will first check if the remote id is allowed to do that, but the permission
/// system is managed not by Rust code of kulfi, but by the fastn server running as the identity
/// server. this allows fastn code to contain a lot of logic. since fastn code is sandboxed, and
/// something end user can easily modify or get from the fastn app marketplace ecosystem, it is a
/// good place to put as much logic as possible into fastn code.
///
/// fastn server will query database etc., will return the ip:port to connect to.
///
/// we have to decide if one tcp connection is one bidirectional stream as disused in protocol.rs.
/// so we will make one tcp connection from this function, and connect the `send` and `recv` streams
/// to tcp connection's `recv` and `send` side respectively.
pub async fn peer_to_tcp(
    addr: &str,
    send: iroh::endpoint::SendStream,
    recv: iroh::endpoint::RecvStream,
) -> eyre::Result<()> {
    // todo: call identity server (fastn server running on behalf of identity
    //       /api/v1/identity/{id}/tcp/ with remote_id and id and get the ip:port
    //       to connect to.

    let stream = tokio::net::TcpStream::connect(addr).await?;
    let (tcp_recv, tcp_send) = tokio::io::split(stream);
    pipe_tcp_stream_over_iroh(tcp_recv, tcp_send, send, recv).await
}

pub async fn pipe_tcp_stream_over_iroh(
    mut tcp_recv: impl tokio::io::AsyncRead + Unpin + Send + 'static,
    tcp_send: impl tokio::io::AsyncWrite + Unpin + Send + 'static,
    mut send: iroh::endpoint::SendStream,
    mut recv: iroh::endpoint::RecvStream,
) -> eyre::Result<()> {
    tracing::trace!("pipe_tcp_stream_over_iroh");

    let t = tokio::spawn(async move {
        let mut t = tcp_send;
        let r = tokio::io::copy(&mut recv, &mut t).await;
        tracing::trace!("piping tcp stream, copy done");
        r.map(|_| ())
    });

    tracing::trace!("copying tcp stream to iroh stream");

    tokio::io::copy(&mut tcp_recv, &mut send).await?;

    tracing::trace!("pipe_tcp_stream_over_iroh copy done");

    send.finish()?;

    tracing::trace!("closed send stream");
    drop(send);

    let r = Ok(t.await??);
    tracing::trace!("pipe_tcp_stream_over_iroh done");
    r
}

pub async fn tcp_to_peer(
    header: kulfi_utils::ProtocolHeader,
    self_endpoint: iroh::Endpoint,
    stream: tokio::net::TcpStream,
    remote_node_id52: &str,
    peer_connections: kulfi_utils::PeerStreamSenders,
    graceful: kulfi_utils::Graceful,
) -> eyre::Result<()> {
    tracing::info!("tcp_to_peer: {remote_node_id52}");

    let (send, recv) = kulfi_utils::get_stream(
        self_endpoint,
        header,
        remote_node_id52.to_string(),
        peer_connections.clone(),
        graceful,
    )
    .await?;

    tracing::info!("got stream");

    let (tcp_recv, tcp_send) = tokio::io::split(stream);
    pipe_tcp_stream_over_iroh(tcp_recv, tcp_send, send, recv).await
}
