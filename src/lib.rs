#![crate_name = "cryptonote"]
#![crate_type = "lib"]

#![feature(box_syntax)]
#![feature(llvm_asm)]
#![feature(repr_simd)]

#[macro_use]
extern crate log;

pub mod hash;
pub mod keccak;
pub mod aes;
pub mod hw_aes;
pub mod sw_aes;
pub mod sse;
pub mod common;
pub mod byte_string;
pub mod u64x2;
