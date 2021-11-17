# crav
Console-based Rust Audio Visualizer

It can run in the terminal but also has a 3D accelerated backend implemented in wgpu.

The terminal backend is currently only working on unix systems, but the wgpu backend should work fine on Windows too.

Because It depends on `audioviz` and how I develop you need to also clone `github.com/BrunoWallner/audioviz` next to the `crav` folder if you want to compile it.

Because of some issues on Linux with [cpal](https://github.com/RustAudio/cpal) the audio backend, I thought of using [pipewire-rs](https://gitlab.freedesktop.org/pipewire/pipewire-rs) for the Linux backend.

## demo
![](/media/demo.png)

## Keyboard shortcuts
* `c` cycles between colors
* `+` increases volume
* `-` decreases volume