use bitcoin::{
    consensus::{deserialize},
    Transaction, OutPoint, 
    Txid, ScriptBuf,
};

#[derive(Debug)]
pub struct DecodedTransaction {
    pub txid: Txid,
    pub version: i32,
    pub lock_time: u32,
    pub inputs: Vec<DecodedInput>,
    pub outputs: Vec<DecodedOutput>,
    pub weight: u64,
    pub vsize: u64,
    pub size: usize,
}

#[derive(Debug)]
pub struct DecodedInput {
    pub previous_output: OutPoint,
    pub script_sig: ScriptBuf,
    pub sequence: u32,
    pub witness: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub struct DecodedOutput {
    pub value: u64,
    pub script_pubkey: ScriptBuf,
}

#[derive(Debug)]
pub struct RawTransactionComponents {
    pub version: String,
    pub marker: Option<String>,
    pub flag: Option<String>,
    pub input_count: String,
    pub inputs: Vec<RawInput>,
    pub output_count: String,
    pub outputs: Vec<RawOutput>,
    pub witness: Vec<RawWitness>,
    pub lock_time: String,
}

#[derive(Debug)]
pub struct RawInput {
    pub txid: String,
    pub vout: String,
    pub scriptsig_size: String,
    pub scriptsig: String,
    pub sequence: String,
}

#[derive(Debug)]
pub struct RawOutput {
    pub amount: String,
    pub scriptpubkey_size: String,
    pub scriptpubkey: String,
}

#[derive(Debug)]
pub struct RawWitness {
    pub stack_items: String,
    pub items: Vec<RawWitnessItem>,
}

#[derive(Debug)]
pub struct RawWitnessItem {
    pub size: String,
    pub item: String,
}

pub struct BitcoinTransactionDecoder;

impl BitcoinTransactionDecoder {
    pub fn new() -> Self {
        Self
    }

    pub fn decode_hex(&self, hex_string: &str) -> Result<DecodedTransaction, Box<dyn std::error::Error>> {
        let clean_hex = hex_string.trim().replace(" ", "").to_lowercase();
        
        let bytes = hex::decode(&clean_hex)?;
        
        self.decode_bytes(&bytes)
    }

    pub fn decode_bytes(&self, bytes: &[u8]) -> Result<DecodedTransaction, Box<dyn std::error::Error>> {
        let tx: Transaction = deserialize(bytes)?;
        
        self.decode_transaction(tx, bytes.len())
    }

    /// Convert a Transaction struct to our decoded format
    fn decode_transaction(&self, tx: Transaction, size: usize) -> Result<DecodedTransaction, Box<dyn std::error::Error>> {
        let txid = tx.compute_txid();
        let weight = tx.weight().to_wu();
        let vsize = tx.vsize() as u64;

        let inputs: Vec<DecodedInput> = tx.input.into_iter().map(|input| {
            DecodedInput {
                previous_output: input.previous_output,
                script_sig: input.script_sig,
                sequence: input.sequence.0,
                witness: input.witness.to_vec(),
            }
        }).collect();

        let outputs: Vec<DecodedOutput> = tx.output.into_iter().map(|output| {
            DecodedOutput {
                value: output.value.to_sat(),
                script_pubkey: output.script_pubkey,
            }
        }).collect();

        Ok(DecodedTransaction {
            txid,
            version: tx.version.0,
            lock_time: tx.lock_time.to_consensus_u32(),
            inputs,
            outputs,
            weight,
            vsize,
            size,
        })
    }

    /// Parse raw transaction hex into detailed components
    pub fn parse_raw_components(&self, hex_string: &str) -> Result<RawTransactionComponents, Box<dyn std::error::Error>> {
        let clean_hex = hex_string.trim().replace(" ", "").to_lowercase();
        let bytes = hex::decode(&clean_hex)?;
        
        let mut cursor = 0;
        
        // Version (4 bytes, little endian)
        let version = hex::encode(&bytes[cursor..cursor + 4]);
        cursor += 4;
        
        // Check for witness flag
        let mut marker = None;
        let mut flag = None;
        let has_witness = bytes[cursor] == 0x00;
        
        if has_witness {
            marker = Some(format!("{:02x}", bytes[cursor]));
            cursor += 1;
            flag = Some(format!("{:02x}", bytes[cursor]));
            cursor += 1;
        }
        
        // Input count (varint)
        let (input_count_val, input_count_bytes) = self.read_varint(&bytes, cursor)?;
        let input_count = hex::encode(&bytes[cursor..cursor + input_count_bytes]);
        cursor += input_count_bytes;
        
        // Parse inputs
        let mut inputs = Vec::new();
        for _ in 0..input_count_val {
            // Previous output hash (32 bytes)
            let mut txid_bytes = bytes[cursor..cursor + 32].to_vec();
            txid_bytes.reverse(); // Reverse for display
            let txid = hex::encode(txid_bytes);
            cursor += 32;
            
            // Previous output index (4 bytes, little endian)
            let vout = hex::encode(&bytes[cursor..cursor + 4]);
            cursor += 4;
            
            // Script sig length (varint)
            let (scriptsig_len, scriptsig_len_bytes) = self.read_varint(&bytes, cursor)?;
            let scriptsig_size = hex::encode(&bytes[cursor..cursor + scriptsig_len_bytes]);
            cursor += scriptsig_len_bytes;
            
            // Script sig
            let scriptsig = if scriptsig_len > 0 {
                hex::encode(&bytes[cursor..cursor + scriptsig_len])
            } else {
                String::new()
            };
            cursor += scriptsig_len;
            
            // Sequence (4 bytes)
            let sequence = hex::encode(&bytes[cursor..cursor + 4]);
            cursor += 4;
            
            inputs.push(RawInput {
                txid,
                vout,
                scriptsig_size,
                scriptsig,
                sequence,
            });
        }
        
        // Output count (varint)
        let (output_count_val, output_count_bytes) = self.read_varint(&bytes, cursor)?;
        let output_count = hex::encode(&bytes[cursor..cursor + output_count_bytes]);
        cursor += output_count_bytes;
        
        // Parse outputs
        let mut outputs = Vec::new();
        for _ in 0..output_count_val {
            // Amount (8 bytes, little endian)
            let amount = hex::encode(&bytes[cursor..cursor + 8]);
            cursor += 8;
            
            // Script pubkey length (varint)
            let (scriptpubkey_len, scriptpubkey_len_bytes) = self.read_varint(&bytes, cursor)?;
            let scriptpubkey_size = hex::encode(&bytes[cursor..cursor + scriptpubkey_len_bytes]);
            cursor += scriptpubkey_len_bytes;
            
            // Script pubkey
            let scriptpubkey = hex::encode(&bytes[cursor..cursor + scriptpubkey_len]);
            cursor += scriptpubkey_len;
            
            outputs.push(RawOutput {
                amount,
                scriptpubkey_size,
                scriptpubkey,
            });
        }
        
        // Parse witness data if present
        let mut witness = Vec::new();
        if has_witness {
            for _ in 0..input_count_val {
                let (stack_items_count, _) = self.read_varint(&bytes, cursor)?;
                let stack_items = format!("{:02x}", stack_items_count);
                cursor += 1; // Assuming single byte for stack items count
                
                let mut items = Vec::new();
                for _ in 0..stack_items_count {
                    let (item_len, item_len_bytes) = self.read_varint(&bytes, cursor)?;
                    let size = format!("{:02x}", item_len);
                    cursor += item_len_bytes;
                    
                    let item = hex::encode(&bytes[cursor..cursor + item_len]);
                    cursor += item_len;
                    
                    items.push(RawWitnessItem { size, item });
                }
                
                witness.push(RawWitness { stack_items, items });
            }
        }
        
        // Lock time (4 bytes)
        let lock_time = hex::encode(&bytes[cursor..cursor + 4]);
        
        Ok(RawTransactionComponents {
            version,
            marker,
            flag,
            input_count,
            inputs,
            output_count,
            outputs,
            witness,
            lock_time,
        })
    }
    
    /// Print raw transaction components using struct formatting
    pub fn print_transaction_components(&self, components: &RawTransactionComponents) {
        println!("=== Raw Transaction Components ===");
        println!("Version: {}", components.version);
        
        if let Some(ref marker) = components.marker {
            println!("Marker: {}", marker);
        }
        if let Some(ref flag) = components.flag {
            println!("Flag: {}", flag);
        }
        
        println!("Input Count: {}", components.input_count);
        
        println!("Inputs:");
        for (i, input) in components.inputs.iter().enumerate() {
            println!("  Input {}:", i);
            println!("    TXID: {}", input.txid);
            println!("    VOUT: {}", input.vout);
            println!("    Script Sig Size: {}", input.scriptsig_size);
            println!("    Script Sig: {}", input.scriptsig);
            println!("    Sequence: {}", input.sequence);
        }
        
        println!("Output Count: {}", components.output_count);
        
        println!("Outputs:");
        for (i, output) in components.outputs.iter().enumerate() {
            println!("  Output {}:", i);
            println!("    Amount: {}", output.amount);
            println!("    Script PubKey Size: {}", output.scriptpubkey_size);
            println!("    Script PubKey: {}", output.scriptpubkey);
        }
        
        if !components.witness.is_empty() {
            println!("Witness:");
            for (i, witness) in components.witness.iter().enumerate() {
                println!("  Witness {}:", i);
                println!("    Stack Items: {}", witness.stack_items);
                for (j, item) in witness.items.iter().enumerate() {
                    println!("    Item {}:", j);
                    println!("      Size: {}", item.size);
                    println!("      Data: {}", item.item);
                }
            }
        }
        
        println!("Lock Time: {}", components.lock_time);
    }
    
    /// Helper function to read variable length integers
    fn read_varint(&self, bytes: &[u8], offset: usize) -> Result<(usize, usize), Box<dyn std::error::Error>> {
        if offset >= bytes.len() {
            return Err("Offset out of bounds".into());
        }
        
        let first_byte = bytes[offset];
        
        match first_byte {
            0x00..=0xfc => Ok((first_byte as usize, 1)),
            0xfd => {
                if offset + 3 > bytes.len() {
                    return Err("Not enough bytes for varint".into());
                }
                let value = u16::from_le_bytes([bytes[offset + 1], bytes[offset + 2]]) as usize;
                Ok((value, 3))
            }
            0xfe => {
                if offset + 5 > bytes.len() {
                    return Err("Not enough bytes for varint".into());
                }
                let value = u32::from_le_bytes([
                    bytes[offset + 1], 
                    bytes[offset + 2], 
                    bytes[offset + 3], 
                    bytes[offset + 4]
                ]) as usize;
                Ok((value, 5))
            }
            0xff => {
                if offset + 9 > bytes.len() {
                    return Err("Not enough bytes for varint".into());
                }
                let value = u64::from_le_bytes([
                    bytes[offset + 1], bytes[offset + 2], bytes[offset + 3], bytes[offset + 4],
                    bytes[offset + 5], bytes[offset + 6], bytes[offset + 7], bytes[offset + 8]
                ]) as usize;
                Ok((value, 9))
            }
        }
    }

    /// Pretty print a decoded transaction
    pub fn print_transaction(&self, decoded: &DecodedTransaction) {
        println!("TXID: {}", decoded.txid);
        println!("Version: {}", decoded.version);
        println!("Lock Time: {}", decoded.lock_time);
        println!("Size: {} bytes", decoded.size);
        println!("Virtual Size: {} vbytes", decoded.vsize);
        println!("Weight: {} WU", decoded.weight);
        println!();

        println!("INPUTS ({}):", decoded.inputs.len());
        for (i, input) in decoded.inputs.iter().enumerate() {
            println!("  Input {}:", i);
            println!("    Previous Output: {}:{}", input.previous_output.txid, input.previous_output.vout);
            println!("    Script Sig: {}", input.script_sig);
            println!("    Sequence: 0x{:08x}", input.sequence);
            
            if !input.witness.is_empty() {
                println!("    Witness ({} items):", input.witness.len());
                for (j, witness_item) in input.witness.iter().enumerate() {
                    println!("      {}: {}", j, hex::encode(witness_item));
                }
            }
            println!();
        }

        println!("OUTPUTS ({}):", decoded.outputs.len());
        for (i, output) in decoded.outputs.iter().enumerate() {
            println!("  Output {}:", i);
            println!("    Value: {} satoshis ({} BTC)", output.value, output.value as f64 / 100_000_000.0);
            println!("    Script PubKey: {}", output.script_pubkey);
            println!();
        }
    }
    
    /// Parse and print transaction components
    pub fn decode_and_print_raw_components(&self, hex_string: &str) -> Result<(), Box<dyn std::error::Error>> {
        let components = self.parse_raw_components(hex_string)?;
        self.print_transaction_components(&components);
        Ok(())
    }
}

// Example usage and tests
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let decoder = BitcoinTransactionDecoder::new();

    // Example raw transaction (this is a mainnet transaction)
    let raw_tx_hex = "01000000000101d7fc103aeb1e32e125959328597717f83c6de279da205de2cd52472f726171040100000000ffffffff02180114000000000017a914aeb0efc1da63629651dc3322c092c6607937c87c87e8af4d7a000000001600141ce75726e812b2fcaf36d6a178ccbfd58a5efcd602483045022100d91d64b5b0326b83d1cfca891a6df291ba975c43c51abfa0f021d9733fe69d6a02206061089696fb44643c4e6e4311304d6d4c41309c10eba835c2835ced06537e960121021b7f2cb05643404c57d0587b48c8d882a454f1040c47cbd31c73d29b599d040100000000";

    println!("=== Original Decoded Transaction ===");
    match decoder.decode_hex(raw_tx_hex) {
        Ok(decoded) => {
            decoder.print_transaction(&decoded);
        }
        Err(e) => {
            eprintln!("Error decoding transaction: {}", e);
        }
    }

    match decoder.decode_and_print_raw_components(raw_tx_hex) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error parsing raw components: {}", e);
        }
    }

    Ok(())
}