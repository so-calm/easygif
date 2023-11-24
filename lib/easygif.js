// @ts-check

const { existsSync } = require("fs");

const binpath =
  "../bin/" +
  { x64: "x64", arm64: "arm64" }[process.arch] +
  "-" +
  { win32: "msvc", linux: "linux" }[process.platform] +
  "-easygif.node";

if (!existsSync(binpath)) {
  process.stderr.write(
    "ERROR: EasyGIF binaries are not found\nPlease install the library correctly, or check your `CWD`",
  );
  process.exit(1);
}

module.exports = require(binpath);
