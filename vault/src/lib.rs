#![cfg_attr(target_os = "solana", feature(asm_experimental_arch))]

#[cfg(test)]
mod tests;


#[cfg(feature = "optimized")]
mod optimized;
#[cfg(feature = "optimized")]
use optimized::*;

#[cfg(feature = "native")]
mod native;
#[cfg(feature = "native")]
use native::*;