// @ts-check

const msvc = require("./msvc");

module.exports = {
  /**
   * @param {string} dirname
   * @returns {Promise<boolean>}
   */
  downloadFfmpeg(dirname) {
    if (process.platform === "win32") return msvc.downloadFfmpeg(dirname);
    // if (process.platform === "linux" && process.arch === "arm64")
    return Promise.resolve(false);
  },
};
