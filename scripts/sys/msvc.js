// @ts-check

const pathJoin = require("path").join;
const { inflateRawSync } = require("zlib");
const { basename } = require("path");
const { writeFileSync, mkdirSync } = require("fs");
const { inspect } = require("util");

const { error, download, formatErr } = require("../essential");

const LOCAL_FILE_SIG = 0x04034b50;
const CENTRAL_FILE_SIG = 0x02014b50;
const DIGITAL_SIG = 0x05054b50;

const FFMPEG =
  "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip";
const FFMPEG_REQUIRED_FILES = [
  "ffmpeg-master-latest-win64-gpl/bin/ffmpeg.exe",
  "ffmpeg-master-latest-win64-gpl/bin/ffprobe.exe",
];

/**
 * @param {string} dirname
 * @returns {Promise<boolean>}
 */
async function downloadFfmpeg(dirname) {
  const ar = await download(FFMPEG);
  if (typeof ar === "number") {
    error("Failed to download the binaries: " + formatErr(ar));
    return false;
  }

  /** @type {[string, Buffer][]} */
  const files = [];
  try {
    let ptr = 0;
    do {
      const sig = ar.readUint32LE(ptr);
      ptr += 18;

      if (sig === DIGITAL_SIG) {
        const dataSize = ar.readUint16LE(ptr);
        ptr += 2 + dataSize;
        continue;
      }

      if (![LOCAL_FILE_SIG, CENTRAL_FILE_SIG].includes(sig)) {
        error("Invalid signature at 0x" + ptr.toString(16).toUpperCase());
        return false;
      }

      const compressedSize = ar.readUint32LE(ptr);
      ptr += 8;
      const filenameLen = ar.readUint16LE(ptr);
      ptr += 2;
      const extraFieldLen = ar.readUint16LE(ptr);
      ptr += 2;

      if (sig !== LOCAL_FILE_SIG) {
        const fileCommentLen = ar.readUint16LE(ptr);
        ptr +=
          16 + compressedSize + filenameLen + extraFieldLen + fileCommentLen;
        continue;
      }

      const filename = ar.subarray(ptr, ptr + filenameLen).toString("utf8");
      ptr += filenameLen + extraFieldLen;

      if (FFMPEG_REQUIRED_FILES.includes(filename)) {
        files.push([
          basename(filename),
          ar.subarray(ptr, ptr + compressedSize),
        ]);
      }
      ptr += compressedSize;
    } while (ptr < ar.length);
  } catch (_) {
    error("Malformed archive");
    return false;
  }

  if (files.length < FFMPEG_REQUIRED_FILES.length) {
    error("Required files are not found");
    return false;
  }

  try {
    mkdirSync(dirname, { recursive: true });
  } catch (reason) {
    error("Failed to create directory " + inspect(dirname) + ": " + reason);
    return false;
  }

  for (const [filename, comp] of files) {
    let buf;
    try {
      buf = inflateRawSync(comp);
    } catch (_) {
      error("Invalid compression " + inspect(filename));
      return false;
    }

    const filepath = pathJoin(dirname, filename);
    try {
      writeFileSync(filepath, buf);
    } catch (reason) {
      error(
        "Failed to create file " + inspect(filepath) + ": " + inspect(reason),
      );
      return false;
    }
  }

  return true;
}

module.exports = { downloadFfmpeg };
