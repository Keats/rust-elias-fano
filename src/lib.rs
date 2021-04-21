use std::error::Error as StdError;
use std::fmt;

mod utils;

use fixedbitset::FixedBitSet;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EliasFano {
    /// The maximum value of the sequence
    universe: u64,
    /// The number of elements in the sequence
    n: u64,
    lower_bits: u64,
    higher_bits_length: u64,
    mask: u64,
    lower_bits_offset: u64,
    bv_len: u64,
    b: FixedBitSet,
    cur_value: u64,
    position: u64,
    high_bits_pos: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    OutOfBounds,
    Unsorted,
    GreaterThanUniverse,
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::OutOfBounds => write!(f, "Index out of range attempted to be accessed"),
            Error::Unsorted => write!(f, "The iterator was not sorted"),
            Error::GreaterThanUniverse => write!(f, "A value greater than the universe was found"),
        }
    }
}

impl EliasFano {
    pub fn new(universe: u64, n: u64) -> EliasFano {
        let lower_bits = if universe > n {
            utils::msb(universe / n)
        } else {
            0
        };
        let higher_bits_length = n + (universe >> lower_bits) + 2;
        let mask = (1_u64 << lower_bits) - 1;
        let lower_bits_offset = higher_bits_length;
        let bv_len = lower_bits_offset + n * (lower_bits as u64);
        let b = FixedBitSet::with_capacity(bv_len as usize);

        EliasFano {
            universe,
            n,
            lower_bits,
            higher_bits_length,
            mask,
            lower_bits_offset,
            bv_len,
            b,
            cur_value: 0,
            position: 0,
            high_bits_pos: 0,
        }
    }

    pub fn compress<'a>(&mut self, elems: impl Iterator<Item = &'a u64>) -> Result<(), Error> {
        let mut last = 0_u64;

        for (i, elem) in elems.enumerate() {
            if i > 0 && *elem < last {
                return Err(Error::Unsorted);
            }

            if *elem > self.universe {
                return Err(Error::GreaterThanUniverse);
            }

            let high = (elem >> self.lower_bits) + i as u64 + 1;
            let low = elem & self.mask;

            self.b.set(high as usize, true);

            let offset = self.lower_bits_offset + (i as u64 * self.lower_bits);
            utils::set_bits(&mut self.b, offset, low, self.lower_bits);

            last = *elem;

            if i == 0 {
                self.cur_value = *elem;
                self.high_bits_pos = high;
            }
        }

        Ok(())
    }

    pub fn visit(&mut self, position: u64) -> Result<u64, Error> {
        if position > self.size() {
            return Err(Error::OutOfBounds);
        }

        if self.position == position {
            return Ok(self.value());
        }

        if position < self.position {
            self.reset();
        }

        let skip = position - self.position;
        let pos = (0..skip).fold(self.high_bits_pos, |pos, _| {
            utils::get_next_set(&self.b, (pos + 1) as usize)
        });

        self.high_bits_pos = (pos - 1) as u64;
        self.position = position;
        self.read_current_value();
        Ok(self.value())
    }

    pub fn next(&mut self) -> Result<u64, Error> {
        self.position += 1;

        if self.position >= self.size() {
            return Err(Error::OutOfBounds);
        }

        self.read_current_value();
        Ok(self.value())
    }

    pub fn skip(&mut self, n: u64) -> Result<u64, Error> {
        let new_pos = self.position() + n;
        self.visit(new_pos)
    }

    pub fn reset(&mut self) {
        self.high_bits_pos = 0;
        self.position = 0;
        self.read_current_value();
    }

    pub fn position(&self) -> u64 {
        self.position
    }

    pub fn value(&self) -> u64 {
        self.cur_value
    }

    pub fn bit_size(&self) -> usize {
        self.b.len()
    }

    pub fn size(&self) -> u64 {
        self.n
    }

    pub fn into_vec(mut self) -> Vec<u64> {
        self.reset();
        let mut vals = Vec::with_capacity(self.size() as usize);
        vals.push(self.value());
        while let Ok(v) = self.next() {
            vals.push(v);
        }
        vals
    }

    fn read_current_value(&mut self) {
        let pos = if self.high_bits_pos > 0 {
            self.high_bits_pos + 1
        } else {
            self.high_bits_pos
        };

        self.high_bits_pos = utils::get_next_set(&self.b, pos as usize) as u64;

        let mut low = 0;
        let offset = self.lower_bits_offset + self.position * self.lower_bits;

        for i in 0..self.lower_bits {
            if self.b.contains((offset + i + 1) as usize) {
                low += 1;
            }
            low <<= 1;
        }
        low >>= 1;

        self.cur_value =
            (((self.high_bits_pos - self.position - 1) << self.lower_bits) | low) as u64;
    }
}

impl fmt::Display for EliasFano {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
    Universe: {:?}
    Elements: {:?}
    Lower_bits: {:?}
    Higher_bits_length: {:?}
    Mask: 0b{:?}
    Lower_bits_offset: {:?}
    Bitvector length: {:?}
",
            self.universe,
            self.n,
            self.lower_bits,
            self.higher_bits_length,
            self.mask,
            self.lower_bits_offset,
            self.bv_len,
        )
    }
}
