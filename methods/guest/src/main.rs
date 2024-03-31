#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

use base64::{engine::general_purpose::STANDARD, Engine};
use risc0_zkvm::guest::env;
use sha2::{Digest, Sha256};

risc0_zkvm::guest::entry!(main);

// Needs to be divisible by 3 to avoid incorrect results from base64 end padding between chunks
const INPUT_CHUNK_MAX_SIZE: usize = 513;
const OUTPUT_CHUNK_MAX_SIZE: usize = INPUT_CHUNK_MAX_SIZE * 4 / 3 + 4;

fn main() {
    let mut input_buf = [0u8; INPUT_CHUNK_MAX_SIZE];
    let mut output_buf = [0u8; OUTPUT_CHUNK_MAX_SIZE];

    let mut input_hasher = Sha256::new();
    let mut output_hasher = Sha256::new();

    loop {
        let input_chunk_size = env::read::<u16>() as usize;
        if input_chunk_size == 0 {
            break;
        }

        let mut input_buf_chunk = &mut input_buf[0..input_chunk_size];
        env::read_slice(&mut input_buf_chunk);

        input_hasher.update(&input_buf_chunk);

        let bytes_written = STANDARD
            .encode_slice(&input_buf_chunk, &mut output_buf)
            .unwrap();

        let output_buf_chunk = &output_buf[0..bytes_written];

        // Print the base64 encoded chunk
        // env::write_slice(&output_buf_chunk);

        output_hasher.update(&output_buf_chunk);

        if input_chunk_size < INPUT_CHUNK_MAX_SIZE {
            break;
        }
    }

    // Print a newline after printing all the base64 chunks
    // env::write_slice(&['\n' as u8]);

    // write public output to the journal
    env::commit_slice(&input_hasher.finalize());
    env::commit_slice(&output_hasher.finalize());
}
