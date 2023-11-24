// @ts-check

const pathJoin = require("path").join;
const { spawn } = require("child_process");
const { existsSync, mkdirSync, writeFileSync } = require("fs");
const { inspect } = require("util");
const { inflateRawSync } = require("zlib");

const { downloadFfmpeg } = require("./sys/sys");
const { version, homepage } = require("../package.json");

const {
  info,
  error,
  success,
  warn,
  ANSI,
  download,
  formatErr,
  INVALID_RESPONSE,
} = require("./essential");

const BINDIR = "./bin";
const FFMPEG = "ffmpeg";
const FFPROBE = "ffprobe";

/**
 * @param {string} cmd
 * @returns {Promise<boolean>}
 */
function checkbin(cmd) {
  return new Promise(resolve => {
    spawn(cmd)
      .once("exit", resolve.bind(null, true))
      .once("error", resolve.bind(null, false));
  });
}

/**
 * @returns {string}
 */
function resolveEasygifBinname() {
  const arch = { arm64: "arm64" }[process.arch] || "x64";
  const plat = { win32: "msvc", darwin: "darwin" }[process.platform] || "linux";
  return arch + "-" + plat + "-easygif.node";
}

/**
 * @returns {Promise<string | null>}
 */
async function checkFfmpeg() {
  if (
    !((await checkbin(FFMPEG)) || (await checkbin(pathJoin(BINDIR, FFMPEG))))
  ) {
    return "FFMPEG binaries not found";
  } else if (
    !((await checkbin(FFPROBE)) || (await checkbin(pathJoin(BINDIR, FFPROBE))))
  ) {
    return "Incomplete FFMPEG installation detected";
  }
  return null;
}

/**
 * @param {string} binname
 * @returns {Promise<string | null>}
 */
async function checkEasygif(binname) {
  if (!existsSync(pathJoin(BINDIR, binname))) {
    return "EasyGIF binaries not found";
  }
  return null;
}

async function entry() {
  const binname = resolveEasygifBinname();

  const ffmpegError = await checkFfmpeg();
  const easygifError = await checkEasygif(binname);

  if (ffmpegError !== null) {
    warn(ffmpegError);
    info("Downloading the latest binaries");
    if (!(await downloadFfmpeg(BINDIR))) {
      error("Failed to download the FFMPEG binaries");
      return;
    }
    if (ANSI) process.stdout.write("\x1b[3F\x1b[J");
    info("FFMPEG binaries downloaded");
  }

  if (easygifError !== null) {
    info(easygifError);
    info("Downloading corresponding binaries");
    if (
      !(await downloadEasygif(
        BINDIR,
        binname,
        homepage + "/releases/download/v" + version + "/" + binname + ".dfl",
      ))
    ) {
      error("Failed to download the EasyGIF binaries");
      return;
    }
    if (ANSI) process.stdout.write("\x1b[3F\x1b[J");
    info("EasyGIF binaries downloaded");
  }

  success("Installation complete");
}

/**
 * @param {string} dirname
 * @param {string} binname
 * @param {string} asset
 * @returns {Promise<boolean>}
 */
async function downloadEasygif(dirname, binname, asset) {
  const dfl = await download(asset);
  if (typeof dfl === "number") {
    if (dfl === INVALID_RESPONSE) {
      error("No available binaries were found");
    } else {
      error("Failed to download the binaries: " + formatErr(dfl));
    }

    return false;
  }

  try {
    mkdirSync(dirname, { recursive: true });
  } catch (reason) {
    error("Failed to create directory " + inspect(dirname) + ": " + reason);
    return false;
  }

  let buf;
  try {
    buf = inflateRawSync(dfl);
  } catch (_) {
    error("Invalid compression " + inspect(asset));
    return false;
  }

  const filepath = pathJoin(dirname, binname);
  try {
    writeFileSync(filepath, buf);
  } catch (reason) {
    error(
      "Failed to create file " + inspect(filepath) + ": " + inspect(reason),
    );
    return false;
  }

  return true;
}

entry();
