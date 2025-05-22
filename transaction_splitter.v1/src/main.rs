use std::convert::TryInto;
use hex;
use serde_json::{json, Value};

#[derive(Debug)]
pub struct Transaction {
    pub version: String,
    pub marker: Option<String>,
    pub flag: Option<String>,
    pub inputcount: String,
    pub inputs: Vec<Input>,
    pub outputcount: String,
    pub outputs: Vec<Output>,
    pub witness: Option<Vec<Witness>>,
    pub locktime: String,
}

#[derive(Debug)]
pub struct Input {
    pub txid: String,
    pub vout: String,
    pub scriptsigsize: String,
    pub scriptsig: String,
    pub sequence: String,
}

#[derive(Debug)]
pub struct Output {
    pub amount: String,
    pub scriptpubkeysize: String,
    pub scriptpubkey: String,
}

#[derive(Debug)]
pub struct Witness {
    pub stackitems: String,
    pub items: Vec<WitnessItem>,
}

#[derive(Debug)]
pub struct WitnessItem {
    pub index: usize,
    pub size: String,
    pub item: String,
}

fn main() {
    let raw_tx_hex = "010000000001019d78d88ba7223285a8f238a8b4a4cfa50e5a8bae1c48ab9c9fdba65726f67b7b0d00000000ffffffff018ea003000000000017a9143761107a6ed37e71cfec61275f175446e67c23a6870247304402202c744bd89c0aa12f8434cf442f0c67ab78ad6a7670e5ec770e5a5e8c67be474b022034dece145972f135e02f7bbc17853133c876d4f7d521de438dd5d13a529f1f05012103365db62d9cf4b19e4dcebb6946763e8048f315d84814f507fa3ca38412044ba200000000"; // Your raw transaction hex
    let raw_tx = hex::decode(raw_tx_hex).expect("Invalid hex");
    let tx = Transaction::parse(&raw_tx).expect("Parsing failed");
    println!("{:#?}", tx);
}

impl Transaction {
    pub fn parse(raw: &[u8]) -> Result<Self, &'static str> {
        let mut index = 0;
        
        // Read version (4 bytes)
        check_remaining(raw, index, 4)?;
        let version = hex::encode(&raw[index..index+4]);
        index += 4;
        
        // Check for segwit marker and flag
        let (marker, flag) = read_segwit_marker(raw, &mut index)?;
        
        // Read inputs
        let input_count = read_compact_size(raw, &mut index)?;
        let inputcount = format!("{:02x}", input_count);
        
        let mut inputs = Vec::with_capacity(input_count);
        for _ in 0..input_count {
            inputs.push(read_input(raw, &mut index)?);
        }
        
        // Read outputs
        let output_count = read_compact_size(raw, &mut index)?;
        let outputcount = format!("{:02x}", output_count);
        
        let mut outputs = Vec::with_capacity(output_count);
        for _ in 0..output_count {
            outputs.push(read_output(raw, &mut index)?);
        }
        
        // Read witness data if this is a segwit transaction
        let witness = if marker.is_some() {
            Some(read_witnesses(raw, input_count, &mut index)?)
        } else {
            None
        };
        
        // Read locktime (4 bytes)
        check_remaining(raw, index, 4)?;
        let locktime = hex::encode(&raw[index..index+4]);
        index += 4;
        
        if index != raw.len() {
            return Err("Extra data after transaction");
        }
        
        Ok(Transaction {
            version,
            marker,
            flag,
            inputcount,
            inputs,
            outputcount,
            outputs,
            witness,
            locktime,
        })
    }
    
    pub fn to_json(&self) -> Value {
        let mut result = json!({
            "version": self.version,
            "inputcount": self.inputcount,
            "inputs": self.inputs.iter().map(|input| {
                json!({
                    "txid": input.txid,
                    "vout": input.vout,
                    "scriptsigsize": input.scriptsigsize,
                    "scriptsig": input.scriptsig,
                    "sequence": input.sequence
                })
            }).collect::<Vec<Value>>(),
            "outputcount": self.outputcount,
            "outputs": self.outputs.iter().map(|output| {
                json!({
                    "amount": output.amount,
                    "scriptpubkeysize": output.scriptpubkeysize,
                    "scriptpubkey": output.scriptpubkey
                })
            }).collect::<Vec<Value>>(),
            "locktime": self.locktime
        });
        
        // Add marker and flag if present
        if let Some(marker) = &self.marker {
            result["marker"] = json!(marker);
        }
        
        if let Some(flag) = &self.flag {
            result["flag"] = json!(flag);
        }
        
        // Add witness data if present
        if let Some(witnesses) = &self.witness {
            let mut witness_array = Vec::new();
            
            for witness in witnesses {
                let mut witness_obj = json!({
                    "stackitems": witness.stackitems
                });
                
                for item in &witness.items {
                    witness_obj[item.index.to_string()] = json!({
                        "size": item.size,
                        "item": item.item
                    });
                }
                
                witness_array.push(witness_obj);
            }
            
            result["witness"] = json!(witness_array);
        }
        
        result
    }
}

fn read_segwit_marker(data: &[u8], index: &mut usize) -> Result<(Option<String>, Option<String>), &'static str> {
    if check_remaining(data, *index, 2).is_err() {
        return Ok((None, None));
    }
    
    let marker = data[*index];
    let flag = data[*index + 1];
    
    if marker == 0x00 && flag >= 0x01 {
        let marker_hex = format!("{:02x}", marker);
        let flag_hex = format!("{:02x}", flag);
        *index += 2;
        Ok((Some(marker_hex), Some(flag_hex)))
    } else {
        Ok((None, None))
    }
}

fn read_compact_size(data: &[u8], index: &mut usize) -> Result<usize, &'static str> {
    check_remaining(data, *index, 1)?;
    let first = data[*index];
    *index += 1;
    
    match first {
        0x00..=0xfc => Ok(first as usize),
        0xfd => read_compact_size_part(data, index, 2),
        0xfe => read_compact_size_part(data, index, 4),
        0xff => read_compact_size_part(data, index, 8),
    }
}

fn read_compact_size_part(data: &[u8], index: &mut usize, bytes: usize) -> Result<usize, &'static str> {
    check_remaining(data, *index, bytes)?;
    let mut buf = [0u8; 8];
    buf[0..bytes].copy_from_slice(&data[*index..*index+bytes]);
    *index += bytes;
    Ok(match bytes {
        2 => u16::from_le_bytes(buf[0..2].try_into().unwrap()) as usize,
        4 => u32::from_le_bytes(buf[0..4].try_into().unwrap()) as usize,
        8 => u64::from_le_bytes(buf).try_into().unwrap(),
        _ => unreachable!(),
    })
}

fn read_input(data: &[u8], index: &mut usize) -> Result<Input, &'static str> {
    // Read txid (32 bytes)
    check_remaining(data, *index, 32)?;
    let txid = hex::encode(&data[*index..*index+32]);
    *index += 32;
    
    // Read vout (4 bytes)
    check_remaining(data, *index, 4)?;
    let vout = hex::encode(&data[*index..*index+4]);
    *index += 4;
    
    // Read scriptsig size
    let scriptsig_size = read_compact_size(data, index)?;
    let scriptsigsize = format!("{:02x}", scriptsig_size);
    
    // Read scriptsig
    check_remaining(data, *index, scriptsig_size)?;
    let scriptsig = if scriptsig_size > 0 {
        hex::encode(&data[*index..*index+scriptsig_size])
    } else {
        String::new()
    };
    *index += scriptsig_size;
    
    // Read sequence (4 bytes)
    check_remaining(data, *index, 4)?;
    let sequence = hex::encode(&data[*index..*index+4]);
    *index += 4;
    
    Ok(Input {
        txid,
        vout,
        scriptsigsize,
        scriptsig,
        sequence,
    })
}

fn read_output(data: &[u8], index: &mut usize) -> Result<Output, &'static str> {
    // Read amount (8 bytes)
    check_remaining(data, *index, 8)?;
    let amount = hex::encode(&data[*index..*index+8]);
    *index += 8;
    
    // Read scriptpubkey size
    let scriptpubkey_size = read_compact_size(data, index)?;
    let scriptpubkeysize = format!("{:02x}", scriptpubkey_size);
    
    // Read scriptpubkey
    check_remaining(data, *index, scriptpubkey_size)?;
    let scriptpubkey = hex::encode(&data[*index..*index+scriptpubkey_size]);
    *index += scriptpubkey_size;
    
    Ok(Output {
        amount,
        scriptpubkeysize,
        scriptpubkey,
    })
}

fn read_witnesses(data: &[u8], input_count: usize, index: &mut usize) -> Result<Vec<Witness>, &'static str> {
    let mut witnesses = Vec::with_capacity(input_count);
    
    for _ in 0..input_count {
        let stack_items = read_compact_size(data, index)?;
        let stackitems = format!("{:02x}", stack_items);
        
        let mut items = Vec::with_capacity(stack_items);
        
        for i in 0..stack_items {
            let item_size = read_compact_size(data, index)?;
            let size = format!("{:02x}", item_size);
            
            check_remaining(data, *index, item_size)?;
            let item = hex::encode(&data[*index..*index+item_size]);
            *index += item_size;
            
            items.push(WitnessItem {
                index: i,
                size,
                item,
            });
        }
        
        witnesses.push(Witness {
            stackitems,
            items,
        });
    }
    
    Ok(witnesses)
}

fn check_remaining(data: &[u8], index: usize, needed: usize) -> Result<(), &'static str> {
    if data.len() < index + needed {
        Err("Insufficient data")
    } else {
        Ok(())
    }
}