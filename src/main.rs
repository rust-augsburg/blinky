#![no_std]
#![no_main]
extern crate panic_halt;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    watchdog::Watchdog,
    Sio,
};
use rp2040_hal as hal;

//defmt is necessary for the info! macro using probe-rs
use defmt::*;
use defmt_rtt as _;

//semihosting is necessary for the hprintln! macro using openOCD
//use cortex_m_semihosting::hprintln;

//This is necessary to load a boot image
//it is places in the beginning of flash see the memory.x file to find the starting address
//for more details of the boot sequence see the section 2.7 int he rp2040 datasheet
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    //hprintln!("Hello OpenOCD"); //this line needs semihosting and works when debugging with openOCD

    //on this block we are preparing the peripherals for configuration
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    //The external oscillator for the Pico board is 12MHz you get this information from the rp2040 datasheet
    let external_xtal_freq_hz = 12_000_000u32;

    //Initialize the peripherals using clock
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    //it creates a delay object using the available system clock from the rp-2040 microcontroller
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    //it creates a pins object that will allow us configure any GPIO available in the rp-2040
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    //it defines that the pin 2 fo rp2040 will be attached to the GPIO peripheral as an output
    //a push pull output is a configuration defined in the rp2040 that allow is to the GPIO physical pin to "high" (1,8V - 3,3V) or low (0V)
    let mut led_pin = pins.gpio28.into_push_pull_output();

    loop {
        //this will set the corresponding bit (bit 2) in the the register SIO: GPIO_OUT_SET(see in rp2040 datasheet) to "high"
        //it will make the the physical pin go to Vcc(1,8V - 3,3V) making the led turn on
        led_pin.set_high().unwrap();
        info!("Led on");

        //block waiting for 1s using internal system ticks
        delay.delay_ms(1000);

        //this will set the corresponding bit (bit 2) in the the register SIO: GPIO_OUT_CLR(see in rp2040 datasheet) to "high"
        //it will make the the physical pin go to Vcc(1,8V - 3,3V) making the led turn on
        led_pin.set_low().unwrap();
        info!("Led off");

        //block waiting for 0,5s using internal system ticks
        delay.delay_ms(500);
    }
}
