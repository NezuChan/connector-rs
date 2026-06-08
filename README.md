[![build](https://github.com/NezuChan/connector-rs/actions/workflows/build.yml/badge.svg)](https://github.com/NezuChan/connector-rs/actions/workflows/build.yml)
[![publish](https://github.com/NezuChan/connector-rs/actions/workflows/publish.yml/badge.svg)](https://github.com/NezuChan/connector-rs/actions/workflows/publish.yml)

# connector-rs (still in development)

High-performance native audio codec implementations for Lavaplayer, written in Rust with JNI bindings.

## Overview

This project provides native implementations of audio codecs (AAC, MP3, Opus, Vorbis) and sample rate conversion for use with Lavaplayer. Built with Rust for memory safety and performance, it serves as a drop-in replacement for lavaplayer-natives.

## Features

- **AAC Decoder**: Using FDK-AAC library
- **MP3 Decoder**: Using mpg123 library
- **Opus Codec**: Full encoder and decoder support
- **Vorbis Decoder**: Ogg Vorbis decoding support
- **Sample Rate Conversion**: Using libsamplerate

## Installation

Supported native platforms:

**Linux x86_64:**

[![](https://img.shields.io/badge/dynamic/xml?url=https%3A%2F%2Fmaven.pkg.github.com%2FNezuChan%2Fconnector-rs%2Forg%2Fnezu%2Fconnector-native-linux-x86-64%2Fmaven-metadata.xml&query=%2F%2Fmetadata%2Fversioning%2Flatest&label=linux-x86-64&logo=linux&logoColor=white&color=blue)](https://github.com/NezuChan/connector-rs/packages)

**Linux ARM64:**

[![](https://img.shields.io/badge/dynamic/xml?url=https%3A%2F%2Fmaven.pkg.github.com%2FNezuChan%2Fconnector-rs%2Forg%2Fnezu%2Fconnector-native-linux-aarch64%2Fmaven-metadata.xml&query=%2F%2Fmetadata%2Fversioning%2Flatest&label=linux-aarch64&logo=linux&logoColor=white&color=blue)](https://github.com/NezuChan/connector-rs/packages)

**Windows x86_64:**

[![](https://img.shields.io/badge/dynamic/xml?url=https%3A%2F%2Fmaven.pkg.github.com%2FNezuChan%2Fconnector-rs%2Forg%2Fnezu%2Fconnector-native-win-x86-64%2Fmaven-metadata.xml&query=%2F%2Fmetadata%2Fversioning%2Flatest&label=win-x86-64&logo=windows&logoColor=white&color=blue)](https://github.com/NezuChan/connector-rs/packages)

**macOS Universal (x86_64 + aarch64):**

[![](https://img.shields.io/badge/dynamic/xml?url=https%3A%2F%2Fmaven.pkg.github.com%2FNezuChan%2Fconnector-rs%2Forg%2Fnezu%2Fconnector-native-darwin%2Fmaven-metadata.xml&query=%2F%2Fmetadata%2Fversioning%2Flatest&label=darwin&logo=apple&logoColor=white&color=blue)](https://github.com/NezuChan/connector-rs/packages)

### Gradle Example

```gradle
repositories {
    maven {
        url = uri("https://maven.pkg.github.com/NezuChan/connector-rs")
        credentials {
            username = project.findProperty("gpr.user") ?: System.getenv("GITHUB_ACTOR")
            password = project.findProperty("gpr.token") ?: System.getenv("GITHUB_TOKEN")
        }
    }
}

dependencies {
    implementation("org.nezu:connector-native-linux-x86-64:VERSION")
    implementation("org.nezu:connector-native-linux-aarch64:VERSION")
    implementation("org.nezu:connector-native-win-x86-64:VERSION")
    implementation("org.nezu:connector-native-darwin:VERSION")
}
```

## Building

### Prerequisites

- Rust toolchain (1.70+)
- Java Development Kit (JDK 8+)
- Gradle
- Native library dependencies (automatically handled by -sys crates)

### Cross-compilation

The project supports multiple target platforms:

- **Linux**: x86_64 (glibc), aarch64 (ARM 64-bit)
- **Windows**: x86_64
- **macOS**: Universal binary (x86_64 + aarch64)

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
