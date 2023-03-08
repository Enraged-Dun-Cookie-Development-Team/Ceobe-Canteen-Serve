use bitmaps::Bitmap;
use bnum::types::U256;

use crate::error::Error;

const BASE_70: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-~._()!*";


pub trait BitmapStringConv {
    type Error;

    fn bitmap_to_string(&self) -> Result<String, Self::Error>;

    fn string_to_bitmap(string: String) -> Result<Self, Self::Error> where Self: Sized;
}

impl BitmapStringConv for Bitmap::<256> {
    type Error = Error;

    fn bitmap_to_string(&self) -> Result<String, Self::Error> {
        // 将字符串转换成索引
        let mut char_to_index: [u8; 127] = [0; 127];
        let index_to_char:Vec<char> = BASE_70.chars().collect();
        let radix: u32 = index_to_char.len().try_into().unwrap();
        let mut i: u8 = 0;
        for c in index_to_char.clone() {
            char_to_index[c as usize] = i;
            i = i + 1;
        }

        // 转换bitmap成u8数组
        let value = U256::from_radix_le(self.as_bytes(), 256).ok_or(Error::LargeThen256)?;
        let bytes = value.to_radix_le(radix);
        let mut result: Vec<char> = Vec::new();
        // 转换为70进制的字符数组
        for b in bytes {
            result.push(index_to_char[b as usize]);
        }

        // str是转换进制后的字符串
        Ok(result.into_iter().collect())
    }

    
    fn string_to_bitmap(string: String) -> Result<Self, Self::Error> where Self: Sized {
        // 将字符串转换成索引
        let mut char_to_index: [u8; 127] = [0; 127];
        let index_to_char:Vec<char> = BASE_70.chars().collect();
        let radix: u32 = index_to_char.len().try_into().unwrap();
        let mut i: u8 = 0;
        for c in index_to_char {
            char_to_index[c as usize] = i;
            i = i + 1;
        }

        // 转换成u8数组
        let mut bytes: Vec<u8> = Vec::new();
        for c in string.chars() {
            bytes.push(char_to_index[c as usize]);
        }
        let value = U256::from_radix_le(&bytes, radix).ok_or(Error::NotConvertBitmap(string.clone()))?;
        let binding = value.to_radix_le(256);
        let bytes_not_complete = binding.as_slice();

        // 转换成有32位的u8数组
        let mut bytes_32: [u8; 32] = [0; 32];
        let (left, _right) = bytes_32.split_at_mut(bytes_not_complete.len());
        left.copy_from_slice(&bytes_not_complete[..]);
        let bytes_complete = &bytes_32[..];

        Bitmap::<256>::try_from(bytes_complete).map_err(|_| Error::NotConvertBitmap(string))
    }

}