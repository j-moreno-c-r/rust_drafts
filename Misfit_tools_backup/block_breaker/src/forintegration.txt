pub struct breaker_by_flags;

pub struct inputs {
    _txid: String,
    _vout: u32,
    _scriptsig_size: u32,
    _scriptsig: String,
    _sequence: u32,
}

pub struct outputs {
    _amount: u32,
    _scriptpubkeysize: u32,
    _scriptpubkey: String,
}

pub struct witness {
    stack_items: u32,
    size: u32,
    item: String,
}

pub struct is_a_transaction {
    pub _version: u32,
    pub _marker: u8,
    pub _flag: u8,
    pub _input_count: u32,
    pub _inputs: inputs,
    pub _output_count: u32,
    pub _outputs: outputs,
    pub witness: witness,
    pub _locktime: u32,
}

pub struct is_a_block {
    pub _version: u32,
    pub _prev_block_hash: String,
    pub _merkle_root: String,
    pub _timestamp: u32,
    pub _bits: u32,
    pub _nonce: u32,
    pub _transaction_count: u32,
    pub _block_header: String,
}

// Transaction processing implementation
pub struct TransactionProcessor;

impl TransactionProcessor {
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

// Block processing implementation
pub struct BlockProcessor;

impl BlockProcessor {
    pub fn process_version(_version: u32) {
        //same logic of the transaction version
    }
    pub fn process_prev_block_hash(_hash: String) {
        //herer i can create just a random block hash?
    }
    pub fn process_merkle_root(_root: String) {
        //here is easy too just add transactions in the root that are not in the block or remove some of tham
    }
    pub fn process_timestamp(_timestamp: u32) {
        //The current time as a Unix timestamp. realy dont know how the people will verify this....
    }
    pub fn process_bits(_bits: u32) {
        //A compact representation of the current target i think i can just change this value and it will be invalid
    }
    pub fn process_nonce(_nonce: u32) {
        //REAL dont know....
    }
    pub fn process_transaction_count(_count: u32) {
        // same logic of the sizes campus
    }
    pub fn process_block_header(_header: String) {
        //here a not literal in the block but just create someting incompatible with the real block
    }
}

impl breaker_by_flags {
    pub fn is_a_transaction(campus: is_a_transaction) {
        TransactionProcessor::invalidate_all(campus);
        TransactionProcessor::invalidate_version(campus._version);
        TransactionProcessor::invalidate_marker(campus._marker);
        TransactionProcessor::invalidate_flag(campus._flag);
        TransactionProcessor::invalidate_input_count(campus._input_count);
        TransactionProcessor::invalidate_inputs(campus._inputs);
        TransactionProcessor::invalidate_output_count(campus._output_count);
        TransactionProcessor::invalidate_outputs(campus._outputs);
        TransactionProcessor::invalidate_witness(campus.witness);
        TransactionProcessor::invalidate_locktime(campus._locktime);
    }

    pub fn is_a_block(campus: is_a_block) {
        BlockProcessor::process_version(campus._version);
        BlockProcessor::process_prev_block_hash(campus._prev_block_hash);
        BlockProcessor::process_merkle_root(campus._merkle_root);
        BlockProcessor::process_timestamp(campus._timestamp);
        BlockProcessor::process_bits(campus._bits);
        BlockProcessor::process_nonce(campus._nonce);
        BlockProcessor::process_transaction_count(campus._transaction_count);
        BlockProcessor::process_block_header(campus._block_header);
    }
}