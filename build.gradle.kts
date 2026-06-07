import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    kotlin("jvm")
    `maven-publish`
}

group = "org.nezu"
version = "1.0.0"

repositories {
    mavenCentral()
}

java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}

Toolchains.ALL.forEach { toolchain ->
    tasks.register<Exec>("compileRust${toolchain.name}") {
        commandLine = listOf("cargo", "build", "--release", "--target", toolchain.rustTarget)

        doLast {
            copy {
                from("target/${toolchain.rustTarget}/release/${toolchain.prefix}connector.${toolchain.extension}")
                into("src/main/resources/${toolchain.destFolder}")
            }
        }
    }
}

tasks.withType<KotlinCompile> {
    kotlinOptions.jvmTarget = "1.8"
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            groupId = "org.nezu"
            artifactId = "connector"
            version = project.version.toString()

            from(components["java"])

            pom {
                name.set("connector-rs")
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
