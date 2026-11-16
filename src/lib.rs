use std::io;

use io::Write;

use arrow_array::Array;
use arrow_array::ArrayRef;
use arrow_array::cast::AsArray;

use arrow_array::BinaryArray;
use arrow_array::StringArray;

pub const NULL_BYTES: [u8; 4] = u32::to_be_bytes(0xffff_ffff);

pub trait BlobLike: Array {
    fn to_bytes(&self, trusted_index: usize) -> &[u8];

    fn to_writer<W>(&self, trusted_index: usize, wtr: &mut W) -> Result<(), io::Error>
    where
        W: Write,
    {
        if self.is_null(trusted_index) {
            wtr.write_all(&NULL_BYTES)?;
            return Ok(());
        }

        let bytes: &[u8] = self.to_bytes(trusted_index);

        let sz: u32 = bytes.len() as u32;
        let szb: [u8; 4] = sz.to_be_bytes();

        wtr.write_all(&szb)?;
        wtr.write_all(bytes)
    }
}

impl BlobLike for &BinaryArray {
    fn to_bytes(&self, trusted_index: usize) -> &[u8] {
        self.value(trusted_index)
    }
}

impl BlobLike for &StringArray {
    fn to_bytes(&self, trusted_index: usize) -> &[u8] {
        self.value(trusted_index).as_bytes()
    }
}

pub fn ref2txt(r: &ArrayRef) -> Option<&StringArray> {
    r.as_string_opt()
}

pub fn ref2bin(r: &ArrayRef) -> Option<&BinaryArray> {
    r.as_bytes_opt()
}
