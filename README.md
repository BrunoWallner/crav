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

## compiling
Because I develop this and [audioviz](https://github.com/BrunoWallner/audioviz) simultaneously you have to clone `github.com/BrunoWallner/audioviz` next to the `crav` folder if you want to compile it.

In the future when audioviz has reached a semi-stable state this will not be a problem anymore.

## Keyboard shortcuts
* `+` increases volume
* `-` decreases volume