const fs = require("fs")
const os = require("os")
const path = require("path")

const binaryName = "apimock"
const platformOrganization = "@apimock-rs"

function srcDestBinaryPath() {
    const platform = os.platform()

    let platformPackage = null
    let extension = ""

    switch (platform) {
        case "linux":
            platformPackage = "bin-linux-x64-gnu"
            break
        case "darwin":
            platformPackage = "bin-darwin-arm64"
            break
        case "win32":
            platformPackage = "bin-win32-x64-msvc"
            extension = ".exe"
            break
        default:
            console.error(`Unsupported platform: ${platform}`)
            process.exit(1)
    }

    const binDir = __dirname
    const srcDir = path.join(binDir, "..", platformOrganization, platformPackage)
    const srcBinary = path.join(srcDir, `${binaryName}${extension}`)
    const destBinary = path.join(binDir, `${binaryName}${extension}`)

    return [srcBinary, destBinary]
}

function linkOrCopy(src, dest) {
    try {
        if (fs.existsSync(dest)) {
            fs.rmSync(dest, { force: true })
        }

        // Try symbolic link first
        fs.symlinkSync(src, dest, "file")
        fs.chmodSync(src, 0o755)
        console.log(`linked ${src} --> ${dest}`)
    } catch (e) {
        // Fallback to file copy
        console.warn(`symlink failed (${e.message}), falling back to copy.`)
        fs.copyFileSync(src, dest)
        fs.chmodSync(dest, 0o755)
        console.log(`copied ${src} --> ${dest}`)
    }
}

const [srcBinary, destBinary] = srcDestBinaryPath()
linkOrCopy(srcBinary, destBinary)
