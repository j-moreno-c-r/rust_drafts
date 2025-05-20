use sha2::{Sha256, Digest};
use hex;
use std::env;
use std::collections::HashSet;

// Simplified data structures with more idiomatic naming
#[derive(Debug, Clone)]
pub struct Transaction {
    pub version: u32,
    pub marker: u8,
    pub flag: u8,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub witness: Option<Witness>,
    pub locktime: u32,
}

#[derive(Debug, Clone)]
pub struct Input {
    pub txid: String,
    pub vout: u32,
    pub script_sig: ScriptSig,
    pub sequence: u32,
}

#[derive(Debug, Clone)]
pub struct Output {
    pub amount: u64,  // Satoshis
    pub script_pubkey: ScriptPubKey,
}

#[derive(Debug, Clone)]
pub struct ScriptSig {
    pub size: u32,
    pub data: String,
}

#[derive(Debug, Clone)]
pub struct ScriptPubKey {
    pub size: u32,
    pub data: String,
}

#[derive(Debug, Clone)]
pub struct Witness {
    pub stack_items: u32,
    pub size: u32,
    pub data: String,
}

// Define available invalidation flags
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum InvalidationFlag {
    Version,
    Marker,
    Flag,
    InputTxid,
    InputVout,
    InputScriptSig,
    InputSequence,
    OutputAmount,
    OutputScriptPubKey,
    WitnessData,
    Locktime,
    All,
}

impl InvalidationFlag {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "version" => Some(Self::Version),
            "marker" => Some(Self::Marker),
            "flag" => Some(Self::Flag),
            "input-txid" | "txid" => Some(Self::InputTxid),
            "input-vout" | "vout" => Some(Self::InputVout),
            "input-script" | "script-sig" => Some(Self::InputScriptSig),
            "input-sequence" | "sequence" => Some(Self::InputSequence),
            "output-amount" | "amount" => Some(Self::OutputAmount),
            "output-script" | "script-pubkey" => Some(Self::OutputScriptPubKey),
            "witness" | "witness-data" => Some(Self::WitnessData),
            "locktime" => Some(Self::Locktime),
            "all" => Some(Self::All),
            _ => None,
        }
    }
}

// Transaction processor with more focused methods and flag support
pub struct TransactionInvalidator;

impl TransactionInvalidator {
    pub fn invalidate(tx: Transaction, flags: &HashSet<InvalidationFlag>) -> Transaction {
        let should_invalidate_all = flags.contains(&InvalidationFlag::All);
        
        Transaction {
            version: if should_invalidate_all || flags.contains(&InvalidationFlag::Version) {
                Self::invalidate_version(tx.version)
            } else {
                tx.version
            },
            marker: if should_invalidate_all || flags.contains(&InvalidationFlag::Marker) {
                Self::invalidate_marker(tx.marker)
            } else {
                tx.marker
            },
            flag: if should_invalidate_all || flags.contains(&InvalidationFlag::Flag) {
                Self::invalidate_flag(tx.flag)
            } else {
                tx.flag
            },
            inputs: tx.inputs.into_iter()
                .map(|input| Self::invalidate_input(input, flags, should_invalidate_all))
                .collect(),
            outputs: tx.outputs.into_iter()
                .map(|output| Self::invalidate_output(output, flags, should_invalidate_all))
                .collect(),
            witness: tx.witness.map(|w| {
                if should_invalidate_all || flags.contains(&InvalidationFlag::WitnessData) {
                    Self::invalidate_witness(w)
                } else {
                    w
                }
            }),
            locktime: if should_invalidate_all || flags.contains(&InvalidationFlag::Locktime) {
                Self::invalidate_locktime(tx.locktime)
            } else {
                tx.locktime
            },
        }
    }

    fn invalidate_version(v: u32) -> u32 { v + 1 }
    fn invalidate_marker(_: u8) -> u8 { 0x11 }
    fn invalidate_flag(_: u8) -> u8 { 0x00 }
    fn invalidate_locktime(lt: u32) -> u32 { u32::MAX - lt }

    fn invalidate_input(input: Input, flags: &HashSet<InvalidationFlag>, invalidate_all: bool) -> Input {
        Input {
            txid: if invalidate_all || flags.contains(&InvalidationFlag::InputTxid) {
                Self::corrupt_hash(&input.txid)
            } else {
                input.txid
            },
            vout: if invalidate_all || flags.contains(&InvalidationFlag::InputVout) {
                input.vout ^ 1  // Flip last bit
            } else {
                input.vout
            },
            script_sig: if invalidate_all || flags.contains(&InvalidationFlag::InputScriptSig) {
                ScriptSig {
                    size: input.script_sig.size + 10,
                    data: Self::corrupt_hex(&input.script_sig.data),
                }
            } else {
                input.script_sig
            },
            sequence: if invalidate_all || flags.contains(&InvalidationFlag::InputSequence) {
                0xFFFFFFFF ^ input.sequence
            } else {
                input.sequence
            },
        }
    }

    fn invalidate_output(output: Output, flags: &HashSet<InvalidationFlag>, invalidate_all: bool) -> Output {
        Output {
            amount: if invalidate_all || flags.contains(&InvalidationFlag::OutputAmount) {
                u64::MAX - output.amount
            } else {
                output.amount
            },
            script_pubkey: if invalidate_all || flags.contains(&InvalidationFlag::OutputScriptPubKey) {
                ScriptPubKey {
                    size: output.script_pubkey.size + 5,
                    data: Self::corrupt_hex(&output.script_pubkey.data),
                }
            } else {
                output.script_pubkey
            },
        }
    }

    fn invalidate_witness(witness: Witness) -> Witness {
        Witness {
            stack_items: witness.stack_items + 1,
            size: witness.size + 8,
            data: Self::corrupt_hex(&witness.data),
        }
    }

    // Helper methods
    fn corrupt_hash(hash: &str) -> String {
        hex::encode(Sha256::digest(hash.as_bytes()))
    }

    fn corrupt_hex(data: &str) -> String {
        let mut chars: Vec<char> = data.chars().collect();
        if !chars.is_empty() {
            chars[0] = if chars[0] == '0' { 'f' } else { '0' };
        }
        chars.into_iter().collect()
    }
}

// Improved display implementation with field highlighting
impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Transaction (Version: {})", self.version)?;
        writeln!(f, "SegWit Marker: {:02x}, Flag: {:02x}", self.marker, self.flag)?;
        
        writeln!(f, "\nInputs ({}):", self.inputs.len())?;
        for (i, input) in self.inputs.iter().enumerate() {
            writeln!(f, "  #{:02}: TXID: {}, VOUT: {}", i+1, input.txid, input.vout)?;
            writeln!(f, "      ScriptSig: {} ({} bytes)", input.script_sig.data, input.script_sig.size)?;
            writeln!(f, "      Sequence: {:08x}", input.sequence)?;
        }

        writeln!(f, "\nOutputs ({}):", self.outputs.len())?;
        for (i, output) in self.outputs.iter().enumerate() {
            let btc = output.amount as f64 / 100_000_000.0;
            writeln!(f, "  #{:02}: {:.8} BTC", i+1, btc)?;
            writeln!(f, "      ScriptPubKey: {} ({} bytes)", output.script_pubkey.data, output.script_pubkey.size)?;
        }

        if let Some(w) = &self.witness {
            writeln!(f, "\nWitness: {} items ({} bytes)", w.stack_items, w.size)?;
            writeln!(f, "     Data: {}...", if w.data.len() > 16 { &w.data[..16] } else { &w.data })?;
        }

        write!(f, "\nLocktime: {}", self.locktime)
    }
}

fn print_usage() {
    println!("Bitcoin Transaction Invalidator");
    println!("Usage: btc-invalidator [FLAGS]");
    println!("\nAvailable flags:");
    println!("  --all           Invalidate all transaction fields");
    println!("  --version       Invalidate transaction version");
    println!("  --marker        Invalidate SegWit marker");
    println!("  --flag          Invalidate SegWit flag");
    println!("  --txid          Invalidate input transaction ID");
    println!("  --vout          Invalidate input vout");
    println!("  --script-sig    Invalidate input script signature");
    println!("  --sequence      Invalidate input sequence number");
    println!("  --amount        Invalidate output amount");
    println!("  --script-pubkey Invalidate output script pubkey");
    println!("  --witness       Invalidate witness data");
    println!("  --locktime      Invalidate transaction locktime");
    println!("  --help          Print this help message");
    println!("\nExample: btc-invalidator --txid --amount --locktime");
}

fn parse_flags() -> HashSet<InvalidationFlag> {
    let args: Vec<String> = env::args().collect();
    let mut flags = HashSet::new();

    // Display help and exit if requested
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_usage();
        std::process::exit(0);
    }

    // No flags provided - show help
    if args.len() <= 1 {
        print_usage();
        std::process::exit(1);
    }

    // Parse all provided flags
    for arg in args.iter().skip(1) {
        if !arg.starts_with("--") {
            continue;
        }

        let flag_str = &arg[2..]; // Remove "--" prefix
        if let Some(flag) = InvalidationFlag::from_str(flag_str) {
            flags.insert(flag);
        } else {
            println!("Warning: Unknown flag '{}' ignored", arg);
        }
    }

    flags
}

fn create_sample_transaction() -> Transaction {
    Transaction {
        version: 2,
        marker: 0,
        flag: 1,
        inputs: vec![Input {
            txid: "a1075db55d416d3ca199f55b6084e2115b9345e16c5cf302fc80e9d5fbf5d48d".into(),
            vout: 0,
            script_sig: ScriptSig {
                size: 22,
                data: "0014123f6562cc08a7991a8e2bfa2d256a05f5032f87".into(),
            },
            sequence: 0xFFFFFFFF,
        }],
        outputs: vec![
            Output {
                amount: 5_000_000_000,  // 50 BTC
                script_pubkey: ScriptPubKey {
                    size: 22,
                    data: "0014c5dae84e4fc7e812f67c72a1109e317bb3a6e7f7".into(),
                },
            },
            Output {
                amount: 2_500_000_000,  // 25 BTC
                script_pubkey: ScriptPubKey {
                    size: 22,
                    data: "0014a316b8c41c3d6a0d2e59e5783eef558f9c7d3a6c".into(),
                },
            },
        ],
        witness: Some(Witness {
            stack_items: 2,
            size: 72,
            data: "304402207515cf1a...".into(),
        }),
        locktime: 0,
    }
}

fn main() {
    // Parse command line flags
    let flags = parse_flags();
    
    if flags.is_empty() {
        println!("No invalidation flags specified. Use --help for usage information.");
        return;
    }

    // Create valid transaction
    let valid_tx = create_sample_transaction();

    // Create invalid version based on specified flags
    let invalid_tx = TransactionInvalidator::invalidate(valid_tx.clone(), &flags);

    // List which fields are being invalidated
    println!("Invalidating the following fields:");
    for flag in &flags {
        if *flag == InvalidationFlag::All {
            println!("  - ALL FIELDS");
            break;
        }
        match flag {
            InvalidationFlag::Version => println!("  - Transaction Version"),
            InvalidationFlag::Marker => println!("  - SegWit Marker"),
            InvalidationFlag::Flag => println!("  - SegWit Flag"),
            InvalidationFlag::InputTxid => println!("  - Input TXIDs"),
            InvalidationFlag::InputVout => println!("  - Input Vouts"),
            InvalidationFlag::InputScriptSig => println!("  - Input Script Signatures"),
            InvalidationFlag::InputSequence => println!("  - Input Sequences"),
            InvalidationFlag::OutputAmount => println!("  - Output Amounts"),
            InvalidationFlag::OutputScriptPubKey => println!("  - Output Script PubKeys"),
            InvalidationFlag::WitnessData => println!("  - Witness Data"),
            InvalidationFlag::Locktime => println!("  - Locktime"),
            _ => {}
        }
    }

    // Display results
    println!("\nValid Transaction:\n{}\n", valid_tx);
    println!("Invalid Transaction:\n{}", invalid_tx);
}