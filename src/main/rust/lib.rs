#![allow(deprecated)]

#[macro_use]
mod macros;
mod util;

#[cfg(feature = "aac")]
pub mod aac;

#[cfg(feature = "opus")]
pub mod opus;

#[cfg(feature = "samplerate")]
pub mod samplerate;

#[cfg(feature = "vorbis")]
pub mod vorbis;

#[cfg(feature = "mp3")]
pub mod mp3;
