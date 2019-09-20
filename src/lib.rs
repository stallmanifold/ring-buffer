#![no_std]
mod ring_buffer;

pub use ring_buffer::*;

#[cfg(test)]
mod tests;