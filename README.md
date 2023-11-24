# EasyGIF

Extract and render GIF frames easily

> This package utilizes `ffmpeg` for extracting and combining frames. Thus,
> you have to [install _it_](#installation) for the package to work properly.
>
> > Python bindings will probably be available soon

### Status: Unstable

And never tested by the author

## FF (Fast-Forward)

- [Toolchain](#toolchain)
- [Installation](#installation)
- [Potential Issues](#potential-issues) **[ Highly recommended to read ]**
- [Release](#release)
- [Source Code](#sources)
- [Issuing](#issuing)
- [Contributing](#contributing)
- [Maintenance](#maintenance)
- [Examples](#examples)

## Toolchain

Currently only `x86_64-windows-msvc` and `x86_64-linux` builds are available
hence I don't see any need to compile binaries compatible with other
architectures and platforms. If you wish any other build to become available,
just [open an issue](#issuing)

## Installation

Clone the repo and run:

```console
$ node scripts/install
```

Alternatively, you may check the
[**Dependency Installation**](#dependency-installation) or
[**Manual Installation** sections](#manual-installation)

### Dependency Installation

You still can install the library as a dependency of your project by installing
it by repo:

```console
$ npm i git@github.com:so-calm/easygif.git
```

### Manual Installation

- Make sure you have [`ffmpeg` binaries](https://ffmpeg.org/download.html) in\
  your `PATH` environment variable
  - Otherwise, you may include _the binaries_ in the `bin` folder for\
    the package to catch up
- Download the corresponding\
  [**EasyGIF** binaries](https://github.com/so-calm/easygif/releases).\
   Make sure that the version of the package matches with the binary version.\
   Once you got _the binary_, just put it into the `bin` folder

## Manual Build

- **WINDOWS** First, get the node library (`node.lib`) from\
  `%LOCALAPPDATA%\node-gyp\Cache\18.12.1\x64` and copy the file to the `lib`\
  directory
- Do `make {target}`, where target is formatted as:\
  `{arch}-{platform}-{feature}`. Available options are: `x64-msvc-node`
- Your built `.node` binaries are located in `artifacts`
- Once you got the binary, you basically put it into the `bin` folder for the\
  library to catch up

## Potential Issues

You may face a few difficulties while installing and using the package. Just be
prepared for them, so you won't waste your time

- `CWD` sensitivity
  - The installation script is sensitive to the `CWD`, so it may install\
    the binaries somewhere it cannot access itself lol
  - Uh.. well the binary itself is also suffering from the same thing
  - **Manual Fix** You just execute the install script and your app from the\
    correct directory duh
  - **You better fix the package than asking users to use it as you want, you d\*\*\*ss!**\
    I'll look into it if it eventually become a larger problem.\
     If you face any issues, or you really want this to be fixed,\
     consider [opening an issue](#issuing)
- Awful error messages

## Release?

> Will this package ever come to [NPM](https://npmjs.com)?

Uhmm.. maybe? Fortunately the package itself is fully capable of doing it.
All the scripts are compatible and ready for an easy release.
However, I don't see any reasons to release it to the registry for everyone.
If you think this package should be there, you may [open an issue](#issuing)

Btw, the `@zargovv/easygif` package is reserved by me (I'm shocked myself)
on **NPM**, so no one can steal it at any point. Have you heard of malwares on
the registry?

In fact I might get lazy to configure the `.npmignore` file and not release
the package at all though

## Sources?

> What's wrong with the source code?

This question is actually irrelevant. Initially, I didn't want to publish the
source code on github, leaving only the scripts and the types/JS bindings. The
reason for that are the native stuff from the [Rust](https://rust-lang.org)
code. It's a nonsense that this kind of an easy library requires that much of
FFI garbage. The sources themself only contain 3 (maybe more, I might forget to
update the readme) \*technically\* one-liner functions. But I'm just so
frustrated that JavaScript is not capable of easy multi-threading in
promise-like behavior, so here is the package that can handle it for you

## Issuing

If you've caught a bug or got a proposal, feel free to
[open an issue](https://github.com/so-calm/easygif/issues).

I'm too lazy to prepare some sort of a guideline. Please just describe issues
as clearly as possible. Much appreciated

## Contributing

This package is quite simple and I don't feel like it may need any major
maintenance. Sorry, but most likely the contributions will just be ignored

## Maintenance

Check out the [Contributing section](#contributing)

## Todos

- [ ] Optimize `Combine.finish()` method. Current implementation is just straight\
       forward reads the resulting buffer to a `Buffer`
  - [ ] Set initial capacity for buf
  - [ ] Make `write` method asynchronous
  - [ ] Read output on the go
- [ ] Cleanup installation on failure
- [ ] Refactor the codebase

## Examples

The most basic usage is described in this section

- A few more examples can be found in [the corresponding directory](examples)

### Parsing

```js
const easygif = require("./lib/easygif");

async function entry() {
  const probe = await easygif.probe("./icon.gif");
  console.log(probe.width);
  console.log(probe.height);
  console.log(probe.fps);

  console.log(probe);
  // It won't give you a s***. The inspection data is not implemented
  // Don't ask me why.
  // Result is: "Probe {}"

  const extract = await easygif.extract(probe, "./icon.gif");
  console.log(extract);
  // Same case: "Extract {}".
  // But in fact there is no much debug information here, so **IT'S FINE**

  let frameCount = 0;

  // Hence extract iterator returns RGBA buffer of the frame, we may use the
  // bytes however we want
  let totalRed = 0;
  let totalGreen = 0;
  let totalBlue = 0;
  let totalAlpha = 0;

  const start = Date.now();
  // We also may inline the extraction
  for (const rgba of await easygif.extract(probe, "./icon.gif")) {
    // The iterator itself is not asynchronous!

    frameCount += 1;
    for (let i = 0; i < rgba.length; ++i) {
      switch (i % 4) {
        case 0:
          totalRed += rgba[i];
          break;
        case 1:
          totalGreen += rgba[i];
          break;
        case 2:
          totalBlue += rgba[i];
          break;
        case 3:
          totalAlpha += rgba[i];
          break;
      }
    }
  }
  console.log(
    "Read and extracted data from " +
      frameCount +
      " frames in " +
      (Date.now() - start) +
      "ms",
  );

  console.log(totalRed + " red");
  console.log(totalGreen + " green");
  console.log(totalBlue + " blue");
  console.log(totalAlpha + " alpha");
}

entry();
```

### Writing

```js
const easygif = require("./lib/easygif");

async function entry() {
  const start = Date.now();
  const combine = await easygif.combine({
    width: 100,
    height: 100,
    fps: 1,
    repeat: 2,
  });
  const framebuf = combine.alloc();
  for (let i = 0; i < 3; ++i) {
    for (let p = 0; p < framebuf.length; ++p) {
      if (p % 4 === 3 || p % 4 === i) framebuf[p] = 255;
      else framebuf[p] = 0;
    }
    combine.write(framebuf);
  }
  const outbuf = await combine.finish();
  console.log("Complete in " + (Date.now() - start) + "ms");
  require("fs").writeFileSync("./out.gif", outbuf);
}

entry();
```
