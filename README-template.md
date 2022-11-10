
`{{ project-name }}`
=={{ '=================================================' | slice: 0 , project-name.size }}


By {{ authors }}.

License: MIT or Apache-2.0.

This project is configured for the 
    `{{ board }}`.

---
Your environment needs to be set up with some development tools, see https://github.com/Rahix/avr-hal to get started.

Executing `cargo run` will build this project, upload it to your board and open a serial console connected to it. 

If this serial console fails to connect to your board see https://crates.io/crates/ravedude for ways to fix that.