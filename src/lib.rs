#![no_std]
#![allow(async_fn_in_trait)]

use core::iter::once;

pub use embedded_hal::i2c::{AddressMode, I2c};

/// I2C operation.
///
/// Several operations can be combined as part of a transaction.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Operation<'a, B>
where
    B: IntoIterator<Item = u8>,
{
    /// Read data into the provided buffer.
    Read(&'a mut [u8]),
    /// Write data from the provided iterator.
    WriteIter(B),
}

pub trait I2cIter<A: AddressMode>: I2c<A> {
    /// Execute the provided operations on the I2C bus.
    ///
    /// Same as `I2c::transaction` but with an interator instead of a slice of operations.
    fn transaction_iter<'a, O, B>(&mut self, address: A, operations: O) -> Result<(), Self::Error>
    where
        O: IntoIterator<Item = Operation<'a, B>>,
        B: IntoIterator<Item = u8>;

    /// Writes bytes to slave with address `address`
    ///
    /// Same as `I2c::write` but with an interator instead of a slice of bytes.
    fn write_iter<B>(&mut self, address: A, bytes: B) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>,
    {
        self.transaction_iter(address, once(Operation::WriteIter(bytes)))
    }

    /// Writes bytes to slave with address `address` and then reads enough bytes to fill `buffer` *in a
    /// single transaction*
    ///
    /// Same as `I2c::write_read` but with an interator instead of a slice of bytes.
    fn write_iter_read<B>(
        &mut self,
        address: A,
        bytes: B,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>,
    {
        self.transaction_iter(
            address,
            [Operation::WriteIter(bytes), Operation::Read(buffer)],
        )
    }
}

/// Exposes trait for async.await use cases. Requires the `async` feature.
#[cfg(feature = "async")]
pub mod non_blocking {
    use core::iter::once;

    pub use embedded_hal_async::i2c::{AddressMode, I2c};

    use crate::Operation;

    pub trait I2cIter<A: AddressMode>: I2c<A> {
        /// Execute the provided operations on the I2C bus.
        ///
        /// Same as `I2c::transaction` but with an interator instead of a slice.
        async fn transaction_iter<'a, O, B>(
            &mut self,
            address: A,
            operations: O,
        ) -> Result<(), Self::Error>
        where
            O: IntoIterator<Item = Operation<'a, B>>,
            B: IntoIterator<Item = u8>;

        /// Writes bytes to slave with address `address`
        ///
        /// Same as `I2c::write` but with an interator instead of a slice of bytes.
        async fn write_iter<B>(&mut self, address: A, bytes: B) -> Result<(), Self::Error>
        where
            B: IntoIterator<Item = u8>,
        {
            self.transaction_iter(address, once(Operation::WriteIter(bytes)))
                .await
        }

        /// Writes bytes to slave with address `address` and then reads enough bytes to fill `buffer` *in a
        /// single transaction*
        ///
        /// Same as `I2c::write_read` but with an interator instead of a slice of bytes.
        async fn write_iter_read<B>(
            &mut self,
            address: A,
            bytes: B,
            buffer: &mut [u8],
        ) -> Result<(), Self::Error>
        where
            B: IntoIterator<Item = u8>,
        {
            self.transaction_iter(
                address,
                [Operation::WriteIter(bytes), Operation::Read(buffer)],
            )
            .await
        }
    }
}
