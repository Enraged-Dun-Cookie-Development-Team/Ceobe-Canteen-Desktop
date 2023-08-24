const path = require("path")

let extension, hasPortable;
switch (process.platform) {
    case "win32":
        extension = "exe"
        hasPortable = true
        break;
    case "darwin":
        extension = "dmg"
        hasPortable = false
        break;
    case "linux":
        extension = "deb"
        hasPortable = true
        break;
    default:
        throw new Error("Unknown platform")
}

let rootDir = path.resolve(__dirname, "../")
let version = require(rootDir + "/package.json").version
let platform = process.platform

const ProjectPath = {
    ROOT_DIR: rootDir,
    DIST_DIR: path.resolve(__dirname, "../dist_electron"),
    VERSION: version,
    PLATFORM: platform,
    RELEASE_PATH: path.resolve(__dirname, "../release"),
    EXTENSION: extension,
    EXE_RELEASE_NAME: `CeobeCanteenDesktop-${version}-${platform}.${extension}`,
    PORTABLE: hasPortable,
    PORTABLE_RELEASE_NAME: `CeobeCanteenDesktop-${version}-${platform}-portable.zip`
}

module.exports = { ProjectPath }