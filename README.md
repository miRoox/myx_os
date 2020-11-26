# Learn [Writing an OS in Rust](https://github.com/phil-opp/blog_os)

## Note

### A Freestanding Rust Binary

### A Minimal Rust Kernel

To run qemu directly in terminal (without SDL), use 

```shell
qemu-system-x86_64 -curses -drive format=raw,file=./target/x86_64-myx_os/debug/bootimage-myx_os.bin
```

> `-curses`
> * Normally, QEMU uses SDL to display the VGA output. With this option, QEMU can display the VGA output when in text mode using a curses/ncurses interface. Nothing is displayed in graphical mode.
