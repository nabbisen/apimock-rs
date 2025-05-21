import { platform, arch } from "process";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

const getPlatformDir = () => {
    const currentPlatform = `${platform()}-${arch()}`

    switch (currentPlatform) {
        case "linux-x64": return "linux-x64-gnu"
        case "darwin-arm64": return "darwin-arm64"
        case "win32-x64": return "win32-x64-msvc"
    }

    throw new Error(`Unsupported platform: ${currentPlatform}`)
}

function install() {
    const platformDir = getPlatformDir()
    const binSrcPath = resolve(__dirname, "..", platformDir, binaryName)
    const binDestDir = resolve(__dirname, "..", "..", ".bin")
    const binDestPath = join(binDestDir, "apimock")

    try {
        if (!existsSync(binSrcPath)) {
            throw new Error(`binary not found at: ${binSrcPath}`)
        }

        mkdirSync(binDestDir, { recursive: true })

        if (platform() === "win32") {
            copyFileSync(binSrcPath, binDestPath + ".exe")
        } else {
            symlinkSync(binSrcPath, binDestPath)
            chmodSync(binSrcPath, 0o755)
        }

        console.log(`app installed to ${binDestPath}`)
    } catch (err) {
        console.error("failed to install app binary:", err)
        process.exit(1)
    }
}

install()
