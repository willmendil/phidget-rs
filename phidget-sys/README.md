# phidget-sys

Low-level unsafe wrpper around the `phidget22` library for interacting with [Phidget](https://www.phidgets.com/) devices.

This is primarily a set of [bindgen](https://crates.io/crates/bindgen)-generated bindings of the `phidget22.h` header file and linkage to the library.


## Generate binding

> You should not have to regenerate the binding, but for future reference, here's the instructions from [bindgen documentation](https://rust-lang.github.io/rust-bindgen/command-line-usage.html)

- Install bindgen-cli

```powershell
cargo install bindgen-cli 
```

- Generate the binding file

```powershell
bindgen "C:\Program Files\Phidgets\Phidget22\phidget22.h" -o .\bindings\phidget22-64.rs 
```

**Specify the location of you header file on your system**. You can use [everything](https://www.voidtools.com/) to locate your file.