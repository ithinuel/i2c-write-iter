#![no_std]

pub use embedded_hal::i2c::{AddressMode, I2c};

pub trait WriteIter<A: AddressMode>: I2c<A> {
        /// Writes bytes obtained form the iterator `bytes`.
        fn write_iter<'a, U>(&'a mut self, address: A, bytes: U) -> Result<(), Self::Error>
        where
            U: IntoIterator<Item = u8> + 'a;
}

pub trait WriteIterRead<A: AddressMode>: I2c<A> {
        /// Writes bytes obtained form the iterator `bytes`, then read to fill `buffer`.
        fn write_iter_read<'a, U>(&'a mut self, address: A, bytes: U) -> Result<(), Self::Error>
        where
            U: IntoIterator<Item = u8> + 'a;
}

/// Exposes trait for async.await use cases. Requires the `async` feature.
#[cfg(feature = "async")]
pub mod non_blocking {
    pub use embedded_hal_async::i2c::{AddressMode, I2c};

    pub trait WriteIter<A: AddressMode>: I2c<A> {
        /// Writes bytes obtained form the iterator `bytes`.
        async fn write_iter<'a, U>(&'a mut self, address: A, bytes: U) -> Result<(), Self::Error>
        where
            U: IntoIterator<Item = u8> + 'a;
    }

    pub trait WriteIterRead<A: AddressMode>: I2c<A> {
        /// Writes bytes obtained form the iterator `bytes`, then read to fill `buffer`.
        async fn write_iter_read<'a, U>(&'a mut self, address: A, bytes: U, read: &mut [u8]) -> Result<(), Self::Error>
        where
            U: IntoIterator<Item = u8> + 'a;
    }
}
