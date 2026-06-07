package org.nezu.connector.gradle

import org.gradle.api.Project

fun Project.getPlatform(triplet: String = targetPlatform) = when {
    triplet.startsWith("x86_64") && "linux" in triplet && "musl" in triplet -> "linux-musl-x86-64"
    triplet.startsWith("i686") && "linux" in triplet && "musl" in triplet -> "linux-musl-x86"
    triplet.startsWith("aarch64") && "linux" in triplet && "musl" in triplet -> "linux-musl-aarch64"
    triplet.startsWith("arm") && "linux" in triplet && "musl" in triplet -> "linux-musl-arm"

    triplet.startsWith("x86_64") && "linux" in triplet -> "linux-x86-64"
    triplet.startsWith("i686") && "linux" in triplet -> "linux-x86"
    triplet.startsWith("aarch64") && "linux" in triplet -> "linux-aarch64"
    triplet.startsWith("armv7") && "linux" in triplet -> "linux-arm"

    triplet.startsWith("x86_64") && "windows" in triplet -> "win-x86-64"
    triplet.startsWith("i686") && "windows" in triplet -> "win-x86"
    triplet.startsWith("aarch64") && "windows" in triplet -> "win-aarch64"

    "darwin" in triplet -> "darwin"

    else -> throw IllegalArgumentException("Unknown platform: $triplet")
}

val Project.targetPlatform: String get() = findProperty("target") as? String ?: "x86_64-unknown-linux-gnu"
