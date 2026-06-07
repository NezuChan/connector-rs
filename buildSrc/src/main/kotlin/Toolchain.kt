data class Toolchain(
    val platform: String,
    val arch: String,
    val extension: String,
    val prefix: String,
    val rustTarget: String,
) {
    val name = "${platform.split('-').joinToString("") { it.capitalize() }}${arch.capitalize()}"
    val destFolder = "natives/$platform${if (arch.isNotBlank()) "-$arch" else ""}"
}
