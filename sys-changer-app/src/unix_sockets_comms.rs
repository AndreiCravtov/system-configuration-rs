use std::error::Error;
use std::io;
use std::path::Path;
use std::time::Duration;
use event_listener::Event;
use futures_util::StreamExt;
use tokio::net::{UnixListener, UnixStream};
use zbus::{connection, Guid};
use crate::zbus_service::{SysChangerAppService, SysChangerAppServiceProxy};

pub async fn run_server<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // bind on socket to begin a listening server
    let unix_socket_path = path.as_ref().to_path_buf();
    let listener = UnixListener::bind(unix_socket_path)?;
    let guid = Guid::generate();

    // on every connection, serve
    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                // this thing here should be thin copy to the "true" service, e.g.
                // clone a thin wrapper around a bunch of Arc-Mutexes around the REAL state
                //
                // so that on ALL new connection you get a brand new copy of the presentation
                // to the same underlying state guarded by arc-mutexes
                let service = SysChangerAppService {
                    name: "SysChangerAppService_p2p_Name".to_string(),
                    done: Event::new(),
                };

                let _connection = connection::Builder::unix_stream(socket)
                    .server(guid.clone())?
                    .serve_at("/org/zbus/SysChangerAppService", service)? // this thing here SHOULD have a constant!!!!
                    .p2p()
                    .build()
                    .await?;
            }
            Err(e) => {
                eprintln!("accept error = {:?}", e);
            }
        }
    }
}

pub async fn run_client<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let socket = UnixStream::connect(path).await?;
    println!("connected to socket");
    let conn = connection::Builder::unix_stream(socket).p2p().build().await?;
    let service = SysChangerAppServiceProxy::builder(&conn)
        .path("/org/zbus/SysChangerAppService")?
        .build().await?;
    let mut greeted_everyone_stream = service.receive_greeted_everyone().await?;

    println!("{}", service.say_hello("foobar").await?);
    println!("{}", service.greeter_name().await?);
    service.set_greeter_name("aiaio".to_string()).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("{}", service.greeter_name().await?);

    loop {
        tokio::select! {
            got_got = greeted_everyone_stream.select_next_some() => {
                println!("signal received {:?}", got_got);
            },
            _ = tokio::time::sleep(Duration::from_secs(1)) => {
                service.go_away().await?;
                println!("time ticking!!")
            }
        }
    }
}