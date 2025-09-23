//! This is the tutorial: https://dbus2.github.io/zbus/service.html

use std::time::Duration;
use event_listener::{Event, Listener};
use futures_util::StreamExt;
use zbus::{connection, interface, Connection};
use zbus::object_server::SignalEmitter;
use tokio::net::UnixStream;
use zbus::{Guid};

pub struct SysChangerAppService {
    pub name: String,
    pub done: Event,
}

#[interface(
    name = "org.zbus.SysChangerAppService",
    proxy(
        gen_blocking = false,
        default_path = "/org/zbus/SysChangerAppService",
        default_service = "org.zbus.SysChangerAppService",
    ),
)]
impl SysChangerAppService {
    async fn say_hello(&self, name: &str) -> String {
        format!("Hello {}!", name)
    }

    // Rude!
    async fn go_away(
        &self,
        #[zbus(signal_emitter)]
        emitter: SignalEmitter<'_>,
    ) -> zbus::fdo::Result<()> {
        emitter.greeted_everyone().await?;
        self.done.notify(1);

        Ok(())
    }

    /// A "GreeterName" property.
    #[zbus(property)]
    async fn greeter_name(&self) -> String {
        self.name.clone()
    }

    /// A setter for the "GreeterName" property.
    ///
    /// Additionally, a `greeter_name_changed` method has been generated for you if you need to
    /// notify listeners that "GreeterName" was updated. It will be automatically called when
    /// using this setter.
    #[zbus(property)]
    async fn set_greeter_name(&mut self, name: String) {
        self.name = name;
    }

    /// A signal; the implementation is provided by the macro.
    #[zbus(signal)]
    async fn greeted_everyone(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;
}

// Although we use `tokio` here, you can use any async runtime of choice.
pub async fn server_main() -> zbus::Result<()> {
    let service = SysChangerAppService {
        name: "SysChangerAppService_Name".to_string(),
        done: Event::new(),
    };
    let done_listener = service.done.listen();
    let connection = connection::Builder::session()?
        .name("org.zbus.SysChangerAppService")?
        .serve_at("/org/zbus/SysChangerAppService", service)?
        .build()
        .await?;

    done_listener.wait();

    // Let's emit the signal again, just for the fun of it.
    connection
        .object_server()
        .interface("/org/zbus/SysChangerAppService")
        .await?
        .greeted_everyone()
        .await?;

    Ok(())
}


pub async fn client_main() -> zbus::Result<()> {
    let conn = Connection::system().await?;
    let service = SysChangerAppServiceProxy::new(&conn).await?;
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

pub async fn client_server_pair() -> zbus::Result<()> {
    let service = SysChangerAppService {
        name: "SysChangerAppService_p2p_Name".to_string(),
        done: Event::new(),
    };

    let guid = Guid::generate();

    let (p0, p1) = UnixStream::pair()?;
    let (client_conn, server_conn) = futures_util::try_join!(
        // Client
        connection::Builder::unix_stream(p0)
            .p2p()
            .build(),
        // Server
        connection::Builder::unix_stream(p1).server(guid)?
            .serve_at("/org/zbus/SysChangerAppService", service)?
            .p2p()
            .build(),
    )?;

    let service = SysChangerAppServiceProxy::new(&client_conn).await?;
    let mut greeted_everyone_stream = service.receive_greeted_everyone().await?;

    Ok(())
}