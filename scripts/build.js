// @ts-check

const { cpSync, writeFileSync, readFileSync, mkdirSync } = require("fs");
const { dirname } = require("path");
const { deflateRawSync } = require("zlib");

mkdirSync(dirname(process.argv[3]), { recursive: true });
cpSync(process.argv[2], process.argv[3]);
writeFileSync(
  process.argv[3] + ".dfl",
  deflateRawSync(readFileSync(process.argv[3])),
);
