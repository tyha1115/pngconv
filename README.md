# pngconv
PNG converter to C source code (with RGB565A dithering)

## Prerequsites
Mac OS
```sh
$ brew install sdl2
$ curl https://sh.rustup.rs -sSf | sh
```

Ubuntu
```sh
$ sudo apt install libsdl2-dev
$ curl https://sh.rustup.rs -sSf | sh
```

## Usage
Run the converter
```sh
$ cargo run sample.png sample.c
$ ./viewer.sh sample.c
```

## License
Distributed under MIT license. See LICENSE file.
