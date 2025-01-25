<h3 align="center">
A <a href="https://github.com/MediaArea/MediaInfoLib">MediaInfo</a> GUI based on <a href="https://github.com/tauri-apps/tauri">tauri</a>.
</h3>

## Features

- Retrieve detailed information for  a wide variety of media files.

## Install

Download from [release](https://github.com/FengZeng/MediaTool/releases).

Or you can build it yourself. Support macOS 10.15+

## Development

You should install CMake, Rust and Nodejs, see [here](https://v2.tauri.app/start/prerequisites/) for more details. Then install Nodejs packages.

```shell
yarn install
```
Update submodule.
```shell
git submodule update --recursive
```

Then run

```shell
yarn tauri dev
```

Or you can build the app

```shell
yarn tauri build --bundles app
```

## Todos

> - Support http(s) video stream
> - Support thumbnail

## Disclaimer

This is a learning project for Rust practice.

## Contributions

Issue and PR welcome!

## Acknowledgement

MediaTool was based on or inspired by these projects and so on:
- [MediaArea/MediaInfoLib](https://github.com/MediaArea/MediaInfoLib): Convenient unified display of the most relevant technical and tag data for video and audio files.
- [tauri-apps/tauri](https://github.com/tauri-apps/tauri): Build smaller, faster, and more secure desktop applications with a web frontend.
- [vitejs/vite](https://github.com/vitejs/vite): Next generation frontend tooling. It's fast!

## License

BSD 2-Clause "Simplified" License. See [License here](./LICENSE) for details.
