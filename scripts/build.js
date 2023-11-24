// @ts-check

const { cpSync, writeFileSync, readFileSync } = require("fs");
const { deflateRawSync } = require("zlib");

cpSync(process.argv[2], process.argv[3]);
writeFileSync(
  process.argv[3] + ".dfl",
  deflateRawSync(readFileSync(process.argv[3])),
);
