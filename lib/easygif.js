// @ts-check

module.exports = require("../bin/" +
  { x64: "x64" }[process.arch] +
  "-" +
  { win32: "msvc", linux: "linux" }[process.platform] +
  "-easygif.node");
