// @ts-check

const { execSync } = require("child_process");
const { get } = require("https");

const ANSI = supportsAnsi();

// Reference: https://github.com/keqingrong/supports-ansi/blob/master/index.js
/**
 * @returns {boolean}
 */
function supportsAnsi() {
  // Check if it is running in the terminal.
  // NOTE: `process.stdout.isTTY` always return undefined on Cygwin.
  // See https://github.com/nodejs/node/issues/3006
  if (!process.stdout.isTTY) {
    return false;
  }

  if (process.platform === "win32") {
    // Be natively supported on Windows 10 after v.1607 ("Anniversery Update",
    // OS build 14393).
    // Reference: https://api.dartlang.org/stable/1.24.3/dart-io/Stdout/supportsAnsiEscapes.html
    const osRelease = require("os").release().split(".");
    if (
      parseInt(osRelease[0], 10) >= 10 && // major version
      parseInt(osRelease[2], 10) >= 14393 // build number
    ) {
      return true;
    }

    // Be supported on MinGW with Mintty.
    // MinGW may not create the environment variable `TERM`.
    if (
      process.platform === "win32" &&
      execSync("uname", { encoding: "utf-8" }).toLowerCase().includes("mingw")
    ) {
      return true;
    }
  }

  // Check if the terminal is of type ANSI/VT100/xterm compatible.
  const pattern = [
    "^xterm", // xterm, PuTTY, Mintty
    "^rxvt", // RXVT
    "^eterm", // Eterm
    "^screen", // GNU screen, tmux
    "^tmux", // tmux
    "^vt100",
    "^vt102",
    "^vt220",
    "^vt320", // DEC VT series
    "ansi", // ANSI
    "scoansi", // SCO ANSI
    "cygwin", // Cygwin, MinGW
    "linux", // Linux console
    "konsole", // Konsole
    "bvterm", // Bitvise SSH Client
  ].join("|");
  const regex = new RegExp(pattern, "i");
  if (
    process.env.TERM &&
    process.env.TERM !== "dumb" &&
    regex.test(process.env.TERM)
  ) {
    return true;
  }

  // ConEmu (from build 120520d) can process ANSI X3.64 when the environment
  // variable `ConEmuANSI` is set to `ON`.
  // See https://conemu.github.io/en/AnsiEscapeCodes.html#Environment_variable
  const isConEmuAnsiOn = (process.env.ConEmuANSI || "").toLowerCase() === "on";
  if (isConEmuAnsiOn) {
    return true;
  }

  // ANSICON provides ANSI escape sequences for Windows console programs. It
  // will create an `ANSICON` environment variable.
  // NOTE: ANSICON supports only a subset of ANSI escape sequences.
  // See https://github.com/adoxa/ansicon/blob/master/ANSI.c#L38
  if (!!process.env.ANSICON) {
    return true;
  }

  return false;
}

const TOO_MANY_REDIRECTS = 1;
const INVALID_LOCATION = 2;
const INVALID_RESPONSE = 5;
const GENERIC = 10;

/**
 * @param {string} url
 * @param {(status: number, res?: import("http").IncomingMessage) => void} cb
 */
function followRedirect(url, cb, count = 0) {
  get(url, res => {
    if (Math.floor((res.statusCode || 0) / 100) == 3) {
      if (count >= 5) {
        cb(TOO_MANY_REDIRECTS);
        return;
      }

      const { location } = res.headers;
      if (typeof location !== "string") {
        cb(INVALID_LOCATION);
        return;
      }

      followRedirect(location, cb, count + 1);
      return;
    }

    cb(0, res);
  });
}

/**
 * @param {number} reason
 * @returns {string}
 */
function formatErr(reason) {
  switch (reason) {
    case TOO_MANY_REDIRECTS:
      return "Too many redirects";
    case INVALID_LOCATION:
      return "Invalid location";
    case INVALID_RESPONSE:
      return "Invalid response";
    default:
      return "Generic";
  }
}

/**
 * @param {number} bytes
 * @returns {string}
 */
function displayBinaryMeasure(bytes) {
  let p = "";
  let n = bytes;
  if (n >= 1024) (n /= 1024), (p = "Ki");
  if (n >= 1024) (n /= 1024), (p = "Mi");
  if (n >= 1024) (n /= 1024), (p = "Gi");
  if (n >= 1024) (n /= 1024), (p = "Ti");
  if (n >= 1024) (n /= 1024), (p = "Pi");
  if (n >= 1024) (n /= 1024), (p = "Ei");
  if (n >= 1024) (n /= 1024), (p = "Zi");
  if (n >= 1024) (n /= 1024), (p = "Yi");
  return String(Math.round(n * 100) / 100) + " " + p + "B";
}

/**
 * @param {number} ms
 * @returns {string}
 */
function displayTimeMeasure(ms) {
  if (ms >= 86_400_000) return String(Math.ceil(ms / 86_400_000)) + "d";
  if (ms >= 3_600_000) return String(Math.ceil(ms / 3_600_000)) + "h";
  if (ms >= 60_000) return String(Math.ceil(ms / 60_000)) + "m";
  if (ms >= 1_000) return String(Math.ceil(ms / 1_000)) + "s";
  return String(Math.ceil(ms)) + "ms";
}

const COLOR_SUCCESS = ANSI ? "\x1b[1;32m" : "";
const COLOR_INFO = ANSI ? "\x1b[1;36m" : "";
const COLOR_WARN = ANSI ? "\x1b[1;33m" : "";
const COLOR_ERR = ANSI ? "\x1b[1;31m" : "";
const COLOR_RESET = ANSI ? "\x1b[0m" : "";

module.exports = {
  ANSI,
  TOO_MANY_REDIRECTS,
  INVALID_LOCATION,
  INVALID_RESPONSE,
  GENERIC,

  success(message) {
    process.stderr.write(
      " ".repeat(5) +
        COLOR_SUCCESS +
        "Success" +
        COLOR_RESET +
        " " +
        message +
        "\n",
    );
  },

  info(message) {
    process.stdout.write(
      " ".repeat(8) + COLOR_INFO + "Info" + COLOR_RESET + " " + message + "\n",
    );
  },

  warn(message) {
    process.stderr.write(
      " ".repeat(8) + COLOR_WARN + "Warn" + COLOR_RESET + " " + message + "\n",
    );
  },

  error(message) {
    process.stderr.write(
      " ".repeat(7) + COLOR_ERR + "Error" + COLOR_RESET + " " + message + "\n",
    );
  },

  formatErr,

  /**
   * @param {string} url
   * @returns {Promise<Buffer | number>}
   */
  async download(url) {
    let nrecv = 0;
    const chunks = [];
    const res = await new Promise(resolve => {
      followRedirect(url, (status, res) => {
        if (!res) {
          resolve(status);
          return;
        }

        const total = Number(res.headers["content-length"]);
        if (
          res.headers["content-type"] !== "application/octet-stream" ||
          Number.isNaN(total)
        ) {
          res.destroy();
          resolve(INVALID_RESPONSE);
          return;
        }
        res.once("error", resolve.bind(null, GENERIC));

        const totalDisplay = displayBinaryMeasure(total);

        const start = Date.now();
        if (ANSI && process.stdout.columns > 30) {
          process.stdout.write(
            "\n\x1b[14G╰\x1b[" +
              (process.stdout.columns - 15) +
              "G╮" +
              "\n" +
              "\x1b[" +
              (process.stdout.columns - 15) +
              "G↓" +
              COLOR_RESET +
              "\n",
          );
        }

        res.on("data", v => {
          chunks.push(v);
          nrecv += v.length;
          if (ANSI && process.stdout.columns > 30) {
            const complete = nrecv >= total;
            const width = process.stdout.columns - 30;
            const progress = (nrecv / total) * width;

            const now = Date.now();
            const speed = nrecv / (now - start || 1);
            const est = (total - nrecv) / speed;

            const speedDisplay = complete
              ? ""
              : displayBinaryMeasure(speed) + "/s";
            const estDisplay = complete ? "" : displayTimeMeasure(est) + " ETA";
            const bDisplay = complete
              ? ""
              : displayBinaryMeasure(nrecv) + " / " + totalDisplay;

            const st = complete
              ? "√"
              : "⠁⠂⠄⡀ "[Math.floor((now - start) / 250) % 5];
            process.stdout.write(
              "\x1b[3F\x1b[K\x1b[1;37m\x1b[13G" +
                (complete ? COLOR_SUCCESS : "\x1b[36m") +
                " " +
                st +
                " \x1b[1;37m" +
                speedDisplay +
                COLOR_RESET +
                "\n" +
                "\x1b[K\x1b[1;37m\x1b[" +
                (13 - estDisplay.length) +
                "G" +
                estDisplay +
                (complete ? COLOR_SUCCESS : "\x1b[36m") +
                " ╰" +
                "─".repeat(Math.floor(progress)) +
                (progress % 1 >= 0.5 ? "╴" : "") +
                " ".repeat(width - Math.round(progress)) +
                "╮ " +
                COLOR_RESET +
                "\n" +
                "\x1b[K\x1b[" +
                (process.stdout.columns - 16 - bDisplay.length) +
                "G\x1b[1;37m" +
                bDisplay +
                " " +
                (complete ? COLOR_SUCCESS : "\x1b[36m") +
                "↓ " +
                COLOR_RESET +
                "\n",
            );
          }
        });
        res.once("end", resolve.bind(null, 0));
        res.once("close", resolve.bind(null, 0));
      });
    });
    return res === 0 ? Buffer.concat(chunks) : res;
  },
};
