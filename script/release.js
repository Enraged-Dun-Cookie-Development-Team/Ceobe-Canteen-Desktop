const archiver = require("archiver")
const assert = require("assert")
const fs = require("fs")
const path = require("path")
const { ProjectPath } = require("./path");
console.log(`Will find extension ${ProjectPath.EXTENSION}, has portable: ${ProjectPath.PORTABLE}`)
console.debug(ProjectPath)
fs.mkdirSync(ProjectPath.RELEASE_PATH, { recursive: true })
// 查找可执行文件
fs.promises.readdir(ProjectPath.DIST_DIR, { withFileTypes: true })
    .then(dirent => {
        let exeFile = dirent
            .filter(dirent => dirent.isFile() && dirent.name.endsWith(ProjectPath.EXTENSION))[0].name;
        console.log(`find executable file ${exeFile}`)
        // 复制可执行文件到 release 目录
        const exeReleaseOutputPath = path.resolve(ProjectPath.RELEASE_PATH, ProjectPath.EXE_RELEASE_NAME)
        fs.copyFileSync(
            path.resolve(ProjectPath.DIST_DIR, exeFile),
            exeReleaseOutputPath
        )
        assert(fs.existsSync(exeReleaseOutputPath), "file not exists")
        console.log(`executable copied to ${exeReleaseOutputPath}`)
        console.log("executable release done")
    })

// 查找免安装文件
fs.promises.readdir(ProjectPath.DIST_DIR, { withFileTypes: true })
    .then(dirent => {
        let portableDir = dirent.filter(dirent => dirent.isDirectory() && dirent.name.endsWith("-unpacked"))[0].name;
        console.log(`find portable file ${portableDir}`)
        // 打包免安装文件到 release 目录
        const portablePackOutputPath = path.resolve(ProjectPath.RELEASE_PATH, ProjectPath.PORTABLE_RELEASE_NAME)
        const archive = archiver('zip', {
            "zlib": { "level": 9 } // Sets the compression level.
        });
        archive.pipe(fs.createWriteStream(portablePackOutputPath));
        const unpackedDir = path.resolve(ProjectPath.DIST_DIR, portableDir)
        archive.directory(unpackedDir, false);
        archive.finalize().then(() => {
            assert(fs.existsSync(portablePackOutputPath), "file not exists")
            console.log(`portable packed to ${portablePackOutputPath}`)
            console.log("portable release done")
        })
    })
