use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{Write, Read};

// Bitcoin network magic bytes (mainnet)
const MAGIC: [u8; 4] = [0xF9, 0xBE, 0xB4, 0xD9];

fn main() -> std::io::Result<()> {
    // Connect to node
    let mut stream = TcpStream::connect("34.90.43.75:8333")?;

    // Construct version message
    let version_payload = build_version_payload();
    let checksum:[u8; 32]= sha256d(&version_payload)[0..4].try_into().unwrap();
    
    // Build full message
    let mut message = Vec::new();
    message.extend(MAGIC);                 // Magic bytes
    message.extend(b"version\0\0\0\0\0");   // Command name (12 bytes)
    message.extend((version_payload.len() as u32).to_le_bytes()); // Payload size
    message.extend(checksum);               // Checksum
    message.extend(version_payload);        // Actual payload

    // Send version message
    stream.write_all(&message)?;
    println!("Sent version message");

    // Read response
    let mut buffer = [0u8; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 { break; }
        
        // Simple message parsing (real implementation would need proper framing)
        if let Some(verack_pos) = buffer[..bytes_read].windows(4).position(|w| w == MAGIC) {
            let command = &buffer[verack_pos+4..verack_pos+16];
            if command.starts_with(b"verack") {
                println!("Received verack!");
                break;
            }
        }
    }

    Ok(())
}

fn build_version_payload() -> Vec<u8> {
    let mut payload = Vec::new();
    
    // Protocol version (70015 = latest before BIP324)
    payload.extend(70015u32.to_le_bytes());
    
    // Services (NODE_NETWORK)
    payload.extend(1u64.to_le_bytes());
    
    // Timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    payload.extend(timestamp.to_le_bytes());
    
    // Receiver address (IPv4 mapped to IPv6)
    payload.extend(1u64.to_le_bytes()); // Services
    payload.extend([0x00; 12]);          // IPv6 prefix
    payload.extend([0xFF, 0xFF]);        // IPv4 marker
    payload.extend([34, 90, 43, 75]);    // IP address
    payload.extend(8333u16.to_be_bytes()); // Port
    
    // Sender address (empty)
    payload.extend(0u64.to_le_bytes());  // Services
    payload.extend([0x00; 16]);          // IPv6
    payload.extend(0u16.to_be_bytes());  // Port
    
    // Nonce
    payload.extend(123456789u64.to_le_bytes());
    
    // User agent
    payload.push(0x00); // Compact size (length 0)
    
    // Start height
    payload.extend(0i32.to_le_bytes());
    
    // Relay flag
    payload.push(0x01); // True
    
    payload
}

// Double SHA-256 implementation for checksum
fn sha256d(data: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let first = Sha256::digest(data);
    let second = Sha256::digest(&first);
    second.into()
}