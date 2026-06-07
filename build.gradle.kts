import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import org.nezu.connector.gradle.getPlatform
import org.nezu.connector.gradle.targetPlatform
import java.io.ByteArrayOutputStream

plugins {
    kotlin("jvm")
    `maven-publish`
}

group = "org.nezu"
version = getGitHash()

fun getGitHash(): String {
    return try {
        val stdout = ByteArrayOutputStream()
        exec {
            commandLine("git", "rev-parse", "--short", "HEAD")
            standardOutput = stdout
        }
        stdout.toString().trim()
    } catch (e: Exception) {
        "1.0.0"
    }
}

repositories {
    mavenCentral()
}

java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}

val platform = getPlatform()

val cargoBuild by tasks.registering(Exec::class) {
    commandLine("cargo", "build", "--release", "--target", targetPlatform)
}

val moveResources by tasks.registering(Copy::class) {
    group = "build"
    dependsOn(cargoBuild)

    from("target/$targetPlatform/release/")

    include {
        it.name == "release" || it.name.endsWith(".so") || it.name.endsWith(".dll") || it.name.endsWith(".dylib")
    }

    into("src/main/resources/natives/$platform")
}

val cleanNatives by tasks.registering(Delete::class) {
    group = "build"
    delete(fileTree("src/main/resources/natives"))
}

tasks.named("clean").configure {
    dependsOn(cleanNatives)
}

tasks.processResources {
    dependsOn(moveResources)

    include {
        it.isDirectory || it.file.parentFile.name == platform
    }
}

tasks.withType<KotlinCompile> {
    kotlinOptions.jvmTarget = "1.8"
}

publishing {
    publications {
        create<MavenPublication>("Release") {
            groupId = "org.nezu"
            artifactId = "connector-native-$platform"
            version = project.version.toString()

            from(components["java"])

            pom {
                name.set("connector-native-$platform")
                description.set("High-performance native audio codec implementations for Lavaplayer, written in Rust with JNI bindings")
                url.set("https://github.com/NezuChan/connector-rs")

                licenses {
                    license {
                        name.set("Apache License 2.0")
                        url.set("https://www.apache.org/licenses/LICENSE-2.0")
                    }
                }

                developers {
                    developer {
                        id.set("nezuchan")
                        name.set("NezuChan")
                    }
                }

                scm {
                    connection.set("scm:git:git://github.com/NezuChan/connector-rs.git")
                    developerConnection.set("scm:git:ssh://github.com/NezuChan/connector-rs.git")
                    url.set("https://github.com/NezuChan/connector-rs")
                }
            }
        }
    }

    repositories {
        maven {
            name = "GitHubPackages"
            url = uri("https://maven.pkg.github.com/NezuChan/connector-rs")
            credentials {
                username = System.getenv("GITHUB_ACTOR")
                password = System.getenv("GITHUB_TOKEN")
            }
        }
    }
}
