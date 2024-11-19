use std::sync::LazyLock;

use bitmaps::Bitmap;
use bnum::types::U256;

use crate::error::Error;

const BASE_70: &[u8] =
    b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-~._()!*";
// 字符转下标
static CHAR_TO_INDEX: LazyLock<[Option<u8>; 127]> = LazyLock::new(|| {
    // u8::MAX字符为ÿ，我们不会用到
    let mut char_to_index: [Option<u8>; 127] = [Option::None; 127];
    for (i, c) in BASE_70.iter().enumerate() {
        char_to_index[*c as usize] = Some(i as u8);
    }
    char_to_index
});
// 字符数量
const RADIX: u32 = BASE_70.len() as u32;

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
        let bytes = value.to_radix_le(RADIX);
        // 转换为70进制的字符
        Ok(bytes
            .into_iter()
            .map(|b| BASE_70[b as usize] as char)
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
                .copied();
            match index.flatten() {
                None => return Err(Error::NotConvertBitmap(string.clone())),
                Some(i) => bytes.push(i),
            }
        }
        let value = U256::from_radix_le(&bytes, RADIX)
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

#[cfg(test)]
mod tests {
    use bitmaps::Bitmap;

    use super::*;

    fn generate_sample_bitmap() -> Bitmap<256> {
        let mut bitmap = Bitmap::<256>::new();
        for i in 1..=40 {
            bitmap.set(i, true); // 设置第i位为 true
        }
        return bitmap;
    }

    #[test]
    fn test_to_base_70() {
        let bitmap = generate_sample_bitmap();

        // 将 bitmap 转换为 base70 字符串
        let base70_string = bitmap.to_base_70().unwrap();

        // 确保 base70 字符串不为空
        assert!(!base70_string.is_empty());

        // 额外验证：可以输出看看转换后的字符串是什么
        println!("Base70 encoded string: {}", base70_string);

        assert_eq!("ugAUrMi".to_owned(), base70_string);
    }

    #[test]
    fn test_from_base_70() {
        let bitmap = generate_sample_bitmap();

        let decoded_bitmap =
            BitmapBase70Conv::from_base_70("ugAUrMi".to_owned()).unwrap();

        assert_eq!(bitmap, decoded_bitmap);
    }

    #[test]
    fn test_invalid_char() {
        let result: Result<Bitmap<256>, _> =
            BitmapBase70Conv::from_base_70("ugAU#@r!()Mi".to_owned());

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_str() {
        let decoded_bitmap: Result<Bitmap<256>, _> =
            BitmapBase70Conv::from_base_70(
                "ugAUr!()MiugAUr!()MiugAUr!()MiugAUr!()MiugAUr!()MiugAUr!\
                 ()MiugAUr!()MiugAUr!()MiugAUr!()Mi"
                    .to_owned(),
            );
        assert!(decoded_bitmap.is_err());
    }
}
