`avr-hal-template`
==================
[`cargo-generate`] template for jumpstarting projects on common AVR
microcontroller boards.  This template supports the following hardware at this
time:

 - Arduino Leonardo
 - Arduino Mega 2560
 - Arduino Nano
 - Arduino Uno
 - SparkFun ProMicro

## Usage
If you don't have them already, install [`cargo-generate`] and [`ravedude`]:

```bash
cargo install cargo-generate
cargo install ravedude
```

Then instanciate this template:

```bash
cargo generate --git https://github.com/Rahix/avr-hal-template.git
```

You will be prompted to select your board - do so and you're ready to roll!
Everything is prepared so you should be able to just

```bash
cargo run
```

and see a blinky flashed to your board!

[`cargo-generate`]: https://github.com/cargo-generate/cargo-generate
[`ravedude`]: https://github.com/Rahix/avr-hal/tree/next/ravedude

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
