#!/usr/bin/env node

const os = require("os")
const { join } = require("path")
const { spawn } = require("child_process")

const binaryName = "apimock"

function binaryPath() {
    const platform = os.platform()
    let extension = ""
    switch (platform) {
        case "win32":
            extension = ".exe"
            break
        default:
    }

    return join(__dirname, `${binaryName}${extension}`)
}

function spawnBinary(binaryPath) {
    // passing command line arguments to the executable
    const args = process.argv.slice(2)

    const child = spawn(binaryPath, args, {
        stdio: "inherit", // sharing std i/o with the parent brings memory efficiency
    })

    child.on("error", (err) => {
        console.error(`failed to start: ${err.message}`)
        process.exit(1)
    })

    child.on("exit", (code, signal) => {
        if (signal) {
            console.error(`exit by signal: ${signal}`)
            process.exit(1)
        }
        process.exit(code)
    })
}

spawnBinary(binaryPath())
