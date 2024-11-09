#![cfg_attr(target_os = "solana", feature(asm_experimental_arch))]

#[cfg(test)]
mod tests;

#[cfg(all(target_os = "solana", feature = "optimized"))]
mod optimized;
#[cfg(all(target_os = "solana", feature = "optimized"))]
use optimized::*;

#[cfg(feature = "native")]
mod native;
#[cfg(feature = "native")]
use native::*;
