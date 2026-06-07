object Toolchains {
    val Darwin = Toolchain("darwin", "", "dylib", "lib", "x86_64-apple-darwin")
    val Darwin8664 = Toolchain("darwin", "x86-64", "dylib", "lib", "x86_64-apple-darwin")
    val DarwinAarch = Toolchain("darwin", "aarch", "dylib", "lib", "aarch64-apple-darwin")
    val LinuxAarch32 = Toolchain("linux", "aarch32", "so", "lib", "armv7-unknown-linux-gnueabi")
    val LinuxAarch64 = Toolchain("linux", "aarch64", "so", "lib", "aarch64-unknown-linux-gnu")
    val LinuxArm = Toolchain("linux", "arm", "so", "lib", "arm-unknown-linux-gnueabi")
    val LinuxArmhf = Toolchain("linux", "armhf", "so", "lib", "arm-unknown-linux-gnueabihf")
    val LinuxMusl8664 = Toolchain("linux-musl", "x86-64", "so", "lib", "x86_64-unknown-linux-musl")
    val Linux8664 = Toolchain("linux", "x86-64", "so", "lib", "x86_64-unknown-linux-gnu")
    val Linux86 = Toolchain("linux", "x86", "so", "", "i686-unknown-linux-gnu")
    val Win8664 = Toolchain("win", "x86-64", "dll", "", "x86_64-pc-windows-msvc")
    val Win86 = Toolchain("win", "x86", "dll", "", "i686-pc-windows-msvc")

    val ALL = listOf(
        Darwin, Darwin8664, DarwinAarch,
        LinuxAarch32, LinuxAarch64, LinuxArm, LinuxArmhf,
        Linux8664, Linux86,
        Win8664, Win86
    )
}
