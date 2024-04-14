Blinky Overview
===============

This is a basic example of a blinky led for the rp2040.
It can be easily adjusted to run on any of the boards Raspberry Pico, Pico H, Pico W, Pico WH

This project is also helpful to test the configuration of your hardware setup.
You either can debug it with openOCD or probe-rs.



References
----------

[Raspberry Pi Pico Documentation](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html)


openOCD
-------

[Open On-Chip Debugger Documentation](https://openocd.org/pages/about.html)
Tested with: Open On-Chip Debugger 0.12.0

+ Install openOCD
```
sudo apt-get install openocd
```
+ Install the [cortex-debug VS Code extension](https://open-vsx.org/extension/marus25/cortex-debug)

Note: the config files "interface/cmsis-dap.cfg" and "target/rp2040.cfg" located under /usr/share/openocd/scripts. For a different location change the attribute configFiles in lauch.json

Probe-rs
--------

[Probe-rs Documentation](https://probe.rs/docs/overview/about-probe-rs/)
Tested with: probe-rs 0.23.0

+ Install the probe-rs tool

```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-installer.sh | sh
```

+ Install the probe-rs extension in VS Code, by installing the latest available from the [Visual Studio Extension Marketplace](https://marketplace.visualstudio.com/items?itemName=probe-rs.probe-rs-debugger&ssr=false)


Other Important VS Code Extensions
-------------------------

+ [rust-analyzer](https://open-vsx.org/vscode/item?itemName=rust-lang.rust-analyzer) - Rust code completions and highlighting

+ [CodeLLDB](https://open-vsx.org/vscode/item?itemName=vadimcn.vscode-lldb) - necessary to debug rust code

+ [Even Better TOML](https://open-vsx.org/vscode/item?itemName=tamasfe.even-better-toml) - Toml files highlighting