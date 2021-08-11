use log::info;
use nadylib::{
    models::{Channel, Message},
    packets::{
        BuddyAddPacket, BuddyRemovePacket, LoginSelectPacket, MsgPrivatePacket,
        OutPrivgrpInvitePacket,
    },
    AOSocket, ReceivedPacket, Result, SocketConfig,
};

async fn run() -> Result<()> {
    let char_name = std::env::var("CHARNAME").expect("CHARNAME not set");
    let username = std::env::var("USERNAME").expect("USERNAME not set");
    let password = std::env::var("PASSWORD").expect("PASSWORD not set");
    let mut sock = AOSocket::connect("chat.d1.funcom.com:7105", SocketConfig::default()).await?;

    while let Ok(packet) = sock.read_packet().await {
        match packet {
            ReceivedPacket::LoginSeed(s) => {
                sock.login(&username, &password, &s.login_seed).await?;
            }
            ReceivedPacket::LoginCharlist(c) => {
                let character = c.characters.iter().find(|i| i.name == char_name).unwrap();
                let pack = LoginSelectPacket {
                    character_id: character.id,
                };
                sock.send(pack).await?;
            }
            ReceivedPacket::LoginOk => info!("Logged in successfully"),
            ReceivedPacket::MsgPrivate(m) => {
                if m.message.text == "register" || m.message.text == "!register" {
                    let id = m.message.sender.unwrap();
                    let pack = BuddyAddPacket {
                        character_id: id,
                        send_tag: String::from("\u{1}"),
                    };
                    sock.send(pack).await?;
                    info!("User {} registered and has been added as a buddy", id);
                } else if m.message.text == "unregister" || m.message.text == "!unregister" {
                    let id = m.message.sender.unwrap();
                    let pack = BuddyRemovePacket { character_id: id };
                    sock.send(pack).await?;
                    info!("User {} unregistered and has been removed as a buddy", id);
                } else if m.message.text == "help" || m.message.text == "!help" {
                    let pack = MsgPrivatePacket {
                        message: Message {
                            sender: None,
                            channel: Channel::Tell(m.message.sender.unwrap()),
                            text: String::from("I provide a private channel for anyone to use. Use !register to get autoinvites and !unregister to unregister"),
                            send_tag: String::from("\u{0}"),
                        }
                    };
                    sock.send(pack).await?;
                }
            }
            ReceivedPacket::BuddyStatus(s) if s.online => {
                let pack = OutPrivgrpInvitePacket {
                    character_id: s.character_id,
                };
                sock.send(pack).await?;
                info!("User {} has logged on and has been invited", s.character_id);
            }
            ReceivedPacket::PrivgrpClijoin(j) => {
                info!("User {} has joined the private channel", j.character_id);
            }
            ReceivedPacket::PrivgrpClipart(p) => {
                info!("User {} has left the private channel", p.character_id);
            }
            ReceivedPacket::PrivgrpMessage(m) => {
                info!("{}: {}", m.message.sender.unwrap(), m.message.text);
            }
            _ => {}
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run())
}
