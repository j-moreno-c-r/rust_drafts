use bitcoin::consensus::Decodable;
use bitcoin::{
    blockdata::{
        block::{Block, Header, Version},
    },
    hash_types::{BlockHash, TxMerkleNode},
    hashes::Hash,
    pow::CompactTarget,
};
use rand::Rng;

// Enum to specify which fields to modify
#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub enum BlockField {
    Version,
    PrevBlockHash,
    MerkleRoot,
    Timestamp,
    Bits,
    Nonce,
    All,
}

// Configuration for block processing
#[derive(Debug, Clone)]
pub struct ProcessingConfig {
    pub fields_to_modify: Vec<BlockField>,
    pub version_override: Option<i32>,
    pub timestamp_offset: Option<i64>, // seconds to add/subtract
    pub randomize_hashes: bool,
}

impl Default for ProcessingConfig {
    fn default() -> Self {
        ProcessingConfig {
            fields_to_modify: vec![BlockField::All],
            version_override: None,
            timestamp_offset: None,
            randomize_hashes: true,
        }
    }
}

// Block processing implementation
pub struct BlockProcessor {
    config: ProcessingConfig,
}

impl BlockProcessor {
    pub fn new(config: ProcessingConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self {
            config: ProcessingConfig::default(),
        }
    }

    // Process the version of the block
    fn process_version(&self, version: i32) -> i32 {
        if let Some(override_version) = self.config.version_override {
            println!("Overriding block version from {} to {}", version, override_version);
            override_version
        } else {
            // Default behavior: set version to maximum valid value
            let modified_version = 0x3FFFFFFF;
            println!("Modified block version from {} to {}", version, modified_version);
            modified_version
        }
    }

    // Process the previous block hash
    fn process_prev_block_hash(&self, hash: &BlockHash) -> BlockHash {
        if self.config.randomize_hashes {
            let random_hash = Self::generate_random_block_hash();
            println!("Modified prev block hash from {} to {}", hash, random_hash);
            random_hash
        } else {
            // Zero out the hash
            let zero_hash = BlockHash::all_zeros();
            println!("Zeroed prev block hash from {} to {}", hash, zero_hash);
            zero_hash
        }
    }

    // Process the merkle root
    fn process_merkle_root(&self, root: &TxMerkleNode) -> TxMerkleNode {
        if self.config.randomize_hashes {
            let random_merkle_root = Self::generate_random_merkle_root();
            println!("Modified merkle root from {} to {}", root, random_merkle_root);
            random_merkle_root
        } else {
            // Zero out the merkle root
            let zero_root = TxMerkleNode::all_zeros();
            println!("Zeroed merkle root from {} to {}", root, zero_root);
            zero_root
        }
    }

    // Process the timestamp
    fn process_timestamp(&self, timestamp: u32) -> u32 {
        let modified_timestamp = if let Some(offset) = self.config.timestamp_offset {
            // Apply custom offset
            (timestamp as i64 + offset).max(0) as u32
        } else {
            // Default: add one year (31,536,000 seconds)
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32;
            current_time.saturating_add(31_536_000)
        };
        
        println!("Modified timestamp from {} to {}", timestamp, modified_timestamp);
        modified_timestamp
    }

    // Process the bits (difficulty target)
    fn process_bits(&self, bits: u32) -> u32 {
        // XOR with mask to modify difficulty
        let modified_bits = bits ^ 0x00FFFFFF;
        println!("Modified bits from 0x{:08x} to 0x{:08x}", bits, modified_bits);
        modified_bits
    }

    // Process the nonce
    fn process_nonce(&self, nonce: u32) -> u32 {
        // Bitwise NOT to invert all bits
        let modified_nonce = !nonce;
        println!("Modified nonce from {} to {}", nonce, modified_nonce);
        modified_nonce
    }

    
    // Check if a specific field should be processed
    fn should_process_field(&self, field: &BlockField) -> bool {
        self.config.fields_to_modify.contains(&BlockField::All) ||
        self.config.fields_to_modify.contains(field)
    }

    // Process the entire block header based on configuration
    pub fn process_block_header(&self, header: &Header) -> Header {
        let mut modified_header = header.clone();

        if self.should_process_field(&BlockField::Version) {
            let new_version = self.process_version(header.version.to_consensus());
            modified_header.version = Version::from_consensus(new_version);
        }

        if self.should_process_field(&BlockField::PrevBlockHash) {
            modified_header.prev_blockhash = self.process_prev_block_hash(&header.prev_blockhash);
        }

        if self.should_process_field(&BlockField::MerkleRoot) {
            modified_header.merkle_root = self.process_merkle_root(&header.merkle_root);
        }

        if self.should_process_field(&BlockField::Timestamp) {
            modified_header.time = self.process_timestamp(header.time);
        }

        if self.should_process_field(&BlockField::Bits) {
            let new_bits = self.process_bits(header.bits.to_consensus());
            modified_header.bits = CompactTarget::from_consensus(new_bits);
        }

        if self.should_process_field(&BlockField::Nonce) {
            modified_header.nonce = self.process_nonce(header.nonce);
        }
        
        println!("Processed block header successfully");
        modified_header
    }
    
    // Helper method to generate a random block hash
    fn generate_random_block_hash() -> BlockHash {
        let mut rng = rand::rng();
        let random_bytes: [u8; 32] = std::array::from_fn(|_| rng.random());
        BlockHash::from_slice(&random_bytes).expect("Failed to create BlockHash from random bytes")
    }
    
    // Helper method to generate a random merkle root
    fn generate_random_merkle_root() -> TxMerkleNode {
        let mut rng = rand::rng();
        let random_bytes: [u8; 32] = std::array::from_fn(|_| rng.random());
        TxMerkleNode::from_slice(&random_bytes).expect("Failed to create TxMerkleNode from random bytes")
    }
    
    // Process an entire block
    pub fn process_block(&self, block: &Block) -> Block {
        let modified_header = self.process_block_header(&block.header);
        
        Block {
            header: modified_header,
            txdata: block.txdata.clone(),
        }
    }

    // Utility method to decode block header from hex string
    pub fn decode_header_from_hex(hex_string: &str) -> Result<Header, Box<dyn std::error::Error>> {
        let bytes = hex::decode(hex_string)?;
        if bytes.len() != 80 {
            return Err(format!("Invalid header length: expected 80 bytes, got {}", bytes.len()).into());
        }
        let header = Header::consensus_decode(&mut &bytes[..])?;
        Ok(header)
    }

    // Utility method to decode block from hex string
    pub fn decode_block_from_hex(hex_string: &str) -> Result<Block, Box<dyn std::error::Error>> {
        let bytes = hex::decode(hex_string)?;
        let block = Block::consensus_decode(&mut &bytes[..])?;
        Ok(block)
    }

    // Create a minimal block from a header (for testing purposes)
    pub fn create_minimal_block_from_header(header: Header) -> Block {
        Block {
            header,
            txdata: vec![], // Empty transaction list
        }
    }

    // Print block header information
    pub fn print_header_info(header: &Header, label: &str) {
        println!("\n=== {} ===", label);
        println!("Version: {}", header.version.to_consensus());
        println!("Previous Block: {}", header.prev_blockhash);
        println!("Merkle Root: {}", header.merkle_root);
        println!("Timestamp: {}", header.time);
        println!("Bits: 0x{:08x}", header.bits.to_consensus());
        println!("Nonce: {}", header.nonce);
        println!("Block Hash: {}", header.block_hash());
    }
}

// Simplified interface for common use cases
pub struct BlockBreaker;

impl BlockBreaker {
    // Break all fields with default settings
    pub fn break_all_fields(block: &Block) -> Block {
        let processor = BlockProcessor::with_default_config();
        processor.process_block(block)
    }

    // Break only specific fields
    pub fn break_specific_fields(block: &Block, fields: Vec<BlockField>) -> Block {
        let config = ProcessingConfig {
            fields_to_modify: fields,
            ..Default::default()
        };
        let processor = BlockProcessor::new(config);
        processor.process_block(block)
    }

    // Break with custom configuration
    pub fn break_with_config(block: &Block, config: ProcessingConfig) -> Block {
        let processor = BlockProcessor::new(config);
        processor.process_block(block)
    }

    // Break header fields and return a minimal block
    pub fn break_header_fields(header: &Header, fields: Vec<BlockField>) -> Block {
        let config = ProcessingConfig {
            fields_to_modify: fields,
            ..Default::default()
        };
        let processor = BlockProcessor::new(config);
        let modified_header = processor.process_block_header(header);
        BlockProcessor::create_minimal_block_from_header(modified_header)
    }
}

// Example usage and tests
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sample block header hex (Bitcoin Genesis Block header)
    let header_hex = "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c";
    
    // Decode the header
    let original_header = BlockProcessor::decode_header_from_hex(header_hex)?;
    
    // Create a minimal block from the header for processing
    let original_block = BlockProcessor::create_minimal_block_from_header(original_header.clone());
    
    // Print original block info
    BlockProcessor::print_header_info(&original_header, "ORIGINAL BLOCK HEADER");
    
    // Example 1: Break all fields
    println!("\n{}" , "=".repeat(50).as_str());
    println!("EXAMPLE 1: Breaking all fields");
    let broken_all = BlockBreaker::break_all_fields(&original_block);
    BlockProcessor::print_header_info(&broken_all.header, "ALL FIELDS BROKEN");
    
    // Example 2: Break only specific fields
    println!("\n{}" , "=".repeat(50).as_str());
    println!("EXAMPLE 2: Breaking only version and nonce");
    let broken_specific = BlockBreaker::break_specific_fields(
        &original_block,
        vec![BlockField::Version, BlockField::Nonce]
    );
    BlockProcessor::print_header_info(&broken_specific.header, "VERSION & NONCE BROKEN");
    
    // Example 3: Custom configuration
    println!("\n{}" , "=".repeat(50).as_str());
    println!("EXAMPLE 3: Custom configuration");
    let custom_config = ProcessingConfig {
        fields_to_modify: vec![BlockField::Timestamp, BlockField::Bits],
        version_override: Some(2),
        timestamp_offset: Some(-86400), // Subtract one day
        randomize_hashes: false,
    };
    let broken_custom = BlockBreaker::break_with_config(&original_block, custom_config);
    BlockProcessor::print_header_info(&broken_custom.header, "CUSTOM CONFIGURATION");
    
    // Example 4: Working directly with headers
    println!("\n{}" , "=".repeat(50).as_str());
    println!("EXAMPLE 4: Working directly with header");
    let broken_header_block = BlockBreaker::break_header_fields(
        &original_header,
        vec![BlockField::MerkleRoot, BlockField::PrevBlockHash]
    );
    BlockProcessor::print_header_info(&broken_header_block.header, "HEADER FIELDS BROKEN");
    
    Ok(())
}
