# crav
Console-based Rust Audio Visualizer

It can run in the terminal but also has a 3D accelerated backend implemented in wgpu.

## demo
![](/media/demo.gif)

## compatibility
The terminal backend is currently only working on unix systems, but the wgpu backend should work fine on Windows too.

### tested Terminals
| terminal emulator | state             |
| ----------------- | ----------------- |
| gnome-terminal    | working, but slow |
| kitty             | perfectly working |
| alacritty         | working, but weird font |
| tty               | working, but limited color and bar height accuracy |
| windows-terminals | not working, but WIP |

## Keyboard shortcuts
* `+` increases volume
* `-` decreases volume
* `m` toggles mirroring