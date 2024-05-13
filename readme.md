Blinky Overview
===============

This is a basic example of a blinky led for the rp2040.
It can be easily adjusted to run on any of the boards Raspberry Pico, Pico H, Pico W, Pico WH

This project is also helpful to test the configuration of your hardware setup.
You either can debug it with openOCD or probe-rs.


References
----------

[Raspberry Pi Pico Documentation](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html)


Probe-rs debugging
------------------

[Probe-rs Documentation](https://probe.rs/docs/overview/about-probe-rs/)
Tested with: probe-rs 0.23.0

+ Install the probe-rs tool

```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-installer.sh | sh
```

+ Install the probe-rs extension in VS Code, by installing the latest available from the [Visual Studio Extension Marketplace](https://marketplace.visualstudio.com/items?itemName=probe-rs.probe-rs-debugger&ssr=false)

Stand-alone flashing
--------------------

It is also possible to flash directly to the target pico board without a debug probe.
For that it is necessary to install the tool elfuf2.

```
cargo install elf2uf2-rs
```

Steps to download flash elf2uf2-rs

1. Disconnect the USB cable
2. Connect the USB again while pressing the bootsel button
3. Execute the command ```cargo run```

    You will see something like this:
    ```
    Found pico uf2 disk /media/user_name/RPI-RP2
    Transfering program to pico
    123.00 KB / 123.00 KB ====================] 100.00 % 129.29 KB/s
    ```

4. After finishing flashing the program will start automatically