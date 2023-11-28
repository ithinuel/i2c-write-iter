# I2C Write Iter

Iterator based write operations where removed from the embedded-hal in:
[https://github.com/rust-embedded/embedded-hal/pull/440]

Yet, iterator based writes come in very handy especially for devices like the sh1107 display controller
where data needs to be interleaved with control bytes.

This crates allows for hal implementations to support those use cases.
HAL implementations requiring DMA backed transactions can still internally use a buffer to fill from
the iterator.
