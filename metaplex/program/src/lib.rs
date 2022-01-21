//! A Token Fraction program for the Solana blockchain.

pub mod deprecated_state;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;
// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

solana_program::declare_id!("D1QcuZQoT3fYhc8c8wqyVkimB18gPGLm3D4hBaZbdwpP");
