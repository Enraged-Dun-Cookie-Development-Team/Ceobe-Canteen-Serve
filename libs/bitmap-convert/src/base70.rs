use bitmaps::Bitmap;
use bnum::types::U256;

use crate::error::Error;

const BASE_70: &str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-~._()!*";

pub trait BitmapBase70Conv {
    type Error;

    fn to_base_70(&self) -> Result<String, Self::Error>;

    fn from_base_70(string: String) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl BitmapBase70Conv for Bitmap<256> {
    type Error = Error;

    fn to_base_70(&self) -> Result<String, Self::Error> {
        // 将字符串转换成索引
        let mut char_to_index: [u8; 127] = [0; 127];
        let index_to_char: Vec<char> = BASE_70.chars().collect();
        let radix = index_to_char.len() as u32;
        for (i, c) in index_to_char.iter().copied().enumerate() {
            char_to_index[c as usize] = i as u8;
        }

        // 转换bitmap成u8数组
        let value = U256::from_radix_le(self.as_bytes(), 256)
            .ok_or(Error::LargeThen256)?;
        let bytes = value.to_radix_le(radix);

        // 转换为70进制的字符
        Ok(bytes
            .into_iter()
            .map(|b| index_to_char[b as usize])
            .collect())
    }

    fn from_base_70(string: String) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        // 将字符串转换成索引
        let mut char_to_index: [u8; 127] = [0; 127];
        let index_to_char: Vec<char> = BASE_70.chars().collect();
        let radix = index_to_char.len() as u32;
        for (i, c) in index_to_char.into_iter().enumerate() {
            char_to_index[c as usize] = i as u8;
        }

        // 转换成u8数组
        let mut bytes: Vec<u8> = Vec::new();
        for c in string.chars() {
            bytes.push(char_to_index[c as usize]);
        }
        let value = U256::from_radix_le(&bytes, radix)
            .ok_or_else(|| Error::NotConvertBitmap(string.clone()))?;
        let binding = value.to_radix_le(256);
        let bytes_not_complete = binding.as_slice();

        // 转换成有32位的u8数组
        let mut bytes_32: [u8; 32] = [0; 32];
        let (left, _right) = bytes_32.split_at_mut(bytes_not_complete.len());
        left.copy_from_slice(bytes_not_complete);
        let bytes_complete = &bytes_32[..];

        Bitmap::<256>::try_from(bytes_complete)
            .map_err(|_| Error::NotConvertBitmap(string))
    }
}
