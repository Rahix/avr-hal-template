#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    {%- if use_uart_console %}
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    {%- endif %}

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    {% case board -%}
      {%- when "Adafruit Trinket" -%}
    let mut led = pins.d1.into_output();
      {%- when
        "Arduino Leonardo",
        "Arduino Mega 2560",
        "Arduino Mega 1280",
        "Arduino Nano",
        "Arduino Nano New Bootloader",
        "Arduino Uno",
        "Nano168",
        "Adafruit Trinket Pro"
      -%}
    let mut led = pins.d13.into_output();
      {%- when "SparkFun ProMicro" -%}
    let mut led = pins.led_rx.into_output();
    {%- endcase %}

    {% if use_uart_console -%}
    ufmt::uwriteln!(&mut serial, "Hello from {{ board }}!").void_unwrap();

    {% endif -%}

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
{%- comment %}
# vim: ft=rust.jinja2
{% endcomment %}
