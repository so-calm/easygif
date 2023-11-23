# EasyGIF

Extract and render GIF frames easily

### Status: Hopefully stable lol
##### Probably the most unstable thing in this package is the README

> This package utilizes `ffmpeg` for extracting and combining frames. Thus, you have to [install _it_](#installation) for the package to work properly.
>
> > Python bindings will probably be available soon

##### Oh, and the install script also can be unstable. Have you seen that garbage? **Please don't.**

## FF (Fast-Forward)

- [Examples](#examples)
- [Installation](#installation)
- [Potential Issues](#potential-issues) **[ Highly recommended to read ]**
- [Release](#release)
- [Source Code](#sources)
- [Issuing](#issuing)
- [Contributing](#contributing)
- [Maintenance](#maintenance)

## Examples

The most basic usage is described in this section

- A few more examples can be found in [the corresponding directory]()

```js
// The example is not loaded, please clear your caches and check if it's available yet
```

## Installation

```console
$ node scripts/install
```

Alternatively, you may check the [**Manual Installation** section](#manual-installation)

### Manual Installation

- Make sure you have [`ffmpeg` binaries](https://ffmpeg.org/download.html) in your `PATH` environment variable
  - Otherwise, you may include _the binaries_ in the `bin` folder for the package to catch up
- Download the corresponding [**EasyGIF** binaries](#easygif). Make sure that the version of the package matches with the binary version. Once you got _the binary_, just put it into the `bin` folder

## Potential Issues

You may face a few difficulties while installing and using the package. Just be prepared for them, so you won't waste your time

- `CWD` sensitivity
  - The installation script is sensitive to the `CWD`, so it may install the binaries somewhere it cannot access itself lol
  - Uh.. well the binary itself is also suffering from the same thing
  - **Manual Fix** You just execute the install script and your app from the correct directory duh
  - **You better fix the package than asking users to use it as you want, you d\*\*\*ss!** I'll look into it if it eventually become a larger problem. If you face any issues, or you really want this to be fixed, consider [opening an issue](#issuing).
- I cannot come around with any others.

## Release?

> Will you ever release this package to [NPM](https://npmjs.com)?

Uhmm.. maybe? Fortunately the package itself is fully capable of doing it. All the scripts are compatible and ready for an easy release. However, I don't see any reasons to release it to the registry for everyone. If you think this package should be there, you may [open an issue](#issuing)

Btw, the `@zargovv/easygif` package is reserved by me (I'm shocked myself) on **NPM**, so no one can steal it at any point. Have you heard of malwares on the registry?

In fact I might get lazy to configure the `.npmignore` file and not release the package at all though

## Sources?

> What's wrong with the source code?

This question is actually irrelevant. Initially, I didn't want to publish the source code on github, leaving only the scripts and the types/JS bindings. The reason for that are the native stuff from the [Rust](https://rust-lang.org) code. It's a nonsense that this kind of an easy library requires that much of FFI garbage. The sources themself only contain 3 (maybe more, I might forget to update the readme) \*technically\* one-liner functions. But I'm just so frustrated that JavaScript is not capable of easy multi-threading in promise-like way, so here is the package that can handle it for you

## Issuing

If you've caught bug or got a proposal, feel free to open an issue.

I'm too lazy to prepare some sort of a guideline. Please just describe issues as clearly as possible. Much appreciated

## Contributing

This package is quite simple and I don't feel like it may need any major maintenance. Sorry, but most likely the contributions will just be ignored

## Maintenance

Check out the [Contributing section](#contributing)
