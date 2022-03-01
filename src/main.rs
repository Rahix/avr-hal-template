#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

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
      {%- when "Arduino Leonardo", "Arduino Mega 2560", "Arduino Nano", "Arduino Uno", "Nano168" -%}
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600); // not strictly necessary, but still useful
    let _ = ufmt::uwriteln!(&mut serial, "hello from Arduino");

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default()); // only needed if you want to sample analog pins

    let mut led = pins.d13.into_output();
      {%- when "SparkFun ProMicro" -%}
    let mut led = pins.led_rx.into_output();
    {%- endcase %}

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
{%- comment %}
# vim: ft=rust.jinja2
{% endcomment %}
