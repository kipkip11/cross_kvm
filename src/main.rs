pub mod clipboard;
pub mod crypto;
pub mod file_engine;
pub mod protocol;

use clap::Parser;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};

#[derive(Parser, Debug)]
#[command(
    name = "cross_kvm",
    version = "0.1.0",
    about = "Fast & Secure LAN KVM for macOS and Windows"
)]
struct Args {
    #[arg(short, long, default_value = "server")]
    mode: String,

    #[arg(short, long, default_value = "0.0.0.0:24900")]
    bind: String,

    #[arg(short, long, default_value = "127.0.0.1:24900")]
    connect: String,

    #[arg(short, long, default_value = "888888")]
    pin: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    info!(
        "🚀 Khởi động Cross-KVM v0.1.0 (Mode: {}, Security: ChaCha20-Poly1305)",
        args.mode
    );

    let secure_channel = Arc::new(crypto::SecureChannel::new(&args.pin));
    let _file_engine = Arc::new(file_engine::FileEngine::new()?);
    let _clipboard_mgr = Arc::new(clipboard::ClipboardManager::new()?);

    info!("📁 Thư mục nhận file an toàn: ~/Downloads/LanDrop/");
    info!(
        "🔒 Kênh mã hóa PSK kích hoạt thành công (PIN: {})",
        args.pin
    );

    if args.mode == "server" {
        let listener = TcpListener::bind(&args.bind).await?;
        info!("🌐 Server KVM đang lắng nghe tại: {}", args.bind);

        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    info!("🔗 Có kết nối mới từ Client: {}", addr);
                    let sec = Arc::clone(&secure_channel);
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(socket, sec).await {
                            error!("Lỗi xử lý kết nối: {:?}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Lỗi chấp nhận kết nối: {:?}", e);
                }
            }
        }
    } else {
        info!("🔌 Đang kết nối tới Server KVM tại: {}", args.connect);
        let mut stream = TcpStream::connect(&args.connect).await?;
        info!("✅ Đã kết nối thành công tới Server!");

        // Gửi gói Handshake
        let handshake = protocol::Packet::Handshake {
            client_name: "MacMini-M4".to_string(),
            os: std::env::consts::OS.to_string(),
            nonce: rand::random(),
        };
        let encoded = bincode::serialize(&handshake)?;
        let encrypted = secure_channel.encrypt(&encoded, &[1u8; 12])?;

        let len = (encrypted.len() as u32).to_be_bytes();
        stream.write_all(&len).await?;
        stream.write_all(&encrypted).await?;
        info!("📤 Đã gửi Handshake mã hóa thành công.");

        // Giữ kết nối
        let mut buf = [0u8; 1024];
        loop {
            let n = stream.read(&mut buf).await?;
            if n == 0 {
                break;
            }
        }
    }

    Ok(())
}

async fn handle_connection(
    mut socket: TcpStream,
    sec: Arc<crypto::SecureChannel>,
) -> anyhow::Result<()> {
    let mut len_buf = [0u8; 4];
    while socket.read_exact(&mut len_buf).await.is_ok() {
        let length = u32::from_be_bytes(len_buf) as usize;
        let mut data_buf = vec![0u8; length];
        socket.read_exact(&mut data_buf).await?;

        if let Ok(decrypted) = sec.decrypt(&data_buf, &[1u8; 12]) {
            if let Ok(packet) = bincode::deserialize::<protocol::Packet>(&decrypted) {
                info!("📨 Nhận gói tin hợp lệ: {:?}", packet);
            }
        }
    }
    Ok(())
}
