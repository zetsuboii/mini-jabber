use futures_util::{SinkExt, StreamExt};
use mini_jabber::xmpp::InitialStreamHeader;
use tokio::io::AsyncReadExt;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use xmlserde::xml_serialize;

#[tokio::main]
async fn main() {
    echo_client().await;
}

async fn echo_client() {
    println!(":: websocket echo client ::");
    let address = "ws://127.0.0.1:9292";
    let url = url::Url::parse(address).expect("invalid address");

    let (ws_stream, _) = connect_async(url).await.expect("failed to connect");
    println!("websocket handshake has been successfully completed");

    let (mut write, mut read) = ws_stream.split();

    let initial_header = InitialStreamHeader {
        from: "zet@mail.com".to_string(),
        to: "su@mail.com".to_string(),
        version: "1.0".to_string(),
        xml_lang: "en".to_string(),
        xmlns: "jabber:client".to_string(),
        xmlns_stream: "http://etherx.jabber.org/streams".to_string(),
    };

    let serialized_header = xml_serialize(initial_header);
    println!("Sending {}", &serialized_header);

    write
        .send(Message::Text(serialized_header))
        .await
        .expect("failed to send initial header");

    read.next().await; // Skip hello message

    let msg = read.next().await.unwrap().unwrap();
    dbg!(msg);

    let msg = read.next().await.unwrap().unwrap();
    dbg!(msg);
}

// Our helper method which will read data from stdin and send it along the
// sender provided.
#[allow(dead_code)]
async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}