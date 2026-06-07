# connector-rs

High-performance native audio codec implementations for Lavaplayer, written in Rust with JNI bindings.

## Overview

This project provides native implementations of audio codecs (AAC, MP3, Opus, Vorbis) and sample rate conversion for use with Lavaplayer. Built with Rust for memory safety and performance, it serves as a drop-in replacement for lavaplayer-natives.

## Features

- **AAC Decoder**: Using FDK-AAC library
- **MP3 Decoder**: Using mpg123 library
- **Opus Codec**: Full encoder and decoder support
- **Vorbis Decoder**: Ogg Vorbis decoding support
- **Sample Rate Conversion**: Using libsamplerate

## Building

### Prerequisites

- Rust toolchain (1.70+)
- Java Development Kit (JDK 8+)
- Gradle
- Native library dependencies (automatically handled by -sys crates)

### Cross-compilation

The project supports multiple target platforms:

- **Darwin**: x86_64, aarch64 (Apple Silicon)
- **Linux**: x86, x86_64, ARM variants, musl
- **Windows**: x86, x86_64 (MSVC)

### Build Commands

```bash
# Build for current platform
cargo build --release

# Build for specific target
cargo build --release --target x86_64-unknown-linux-gnu

# Build all targets via Gradle
./gradlew compileRustDarwin8664
./gradlew compileRustLinux8664
# ... etc
```

## Usage

This library is designed to be a drop-in replacement for `lavaplayer-natives`. Simply include the compiled native libraries in your classpath, and Lavaplayer will automatically use them.

## Architecture

The codebase follows a clean, flat structure:

- **macros.rs**: JNI helper macros for safe pointer handling
- **util.rs**: Buffer access utilities
- **aac.rs, mp3.rs, opus.rs, vorbis.rs, samplerate.rs**: Individual codec implementations

All codecs share consistent patterns:
- `create()`: Initialize codec instance
- `process/encode/decode()`: Process audio data
- `destroy()`: Clean up resources

## Safety

This implementation prioritizes safety through:
- Rust's memory safety guarantees where applicable
- Careful handling of JNI boundaries
- Proper resource cleanup
- Explicit error handling

## License

Licensed under Apache License 2.0
