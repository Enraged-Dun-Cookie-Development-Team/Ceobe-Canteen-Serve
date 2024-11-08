use bitmaps::Bitmap;
use bnum::types::U256;
use once_cell::sync::Lazy;

use crate::error::Error;

const BASE_70: &str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-~._()!*";
// 字符转下标
static CHAR_TO_INDEX: Lazy<[u8; 127]> = Lazy::new(|| {
    let mut char_to_index: [u8; 127] = [0; 127];
    let index_to_char = BASE_70.chars().collect::<Vec<char>>();
    for (i, c) in index_to_char.iter().enumerate() {
        char_to_index[*c as usize] = i as u8;
    }
    char_to_index
});
// 下标转字符
static INDEX_TO_CHAR: Lazy<Vec<char>> =
    Lazy::new(|| BASE_70.chars().collect());
// 字符数量
static RADIX: Lazy<u32> = Lazy::new(|| INDEX_TO_CHAR.len() as u32);

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
        // 转换bitmap成u8数组
        let value = U256::from_radix_le(self.as_bytes(), 256)
            .ok_or(Error::LargeThen256)?;
        let bytes = value.to_radix_le(*RADIX);

        // 转换为70进制的字符
        Ok(bytes
            .into_iter()
            .map(|b| INDEX_TO_CHAR[b as usize])
            .collect())
    }

    fn from_base_70(string: String) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        // 转换成u8数组
        let mut bytes: Vec<u8> = Vec::new();
        for c in string.chars() {
            let index = CHAR_TO_INDEX
                .get(c as usize)
                .ok_or(Error::NotConvertBitmap(string.clone()))?;
            bytes.push(*index);
        }
        let value = U256::from_radix_le(&bytes, *RADIX)
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
