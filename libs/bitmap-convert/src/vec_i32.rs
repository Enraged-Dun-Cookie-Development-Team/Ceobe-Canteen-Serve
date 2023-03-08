use bitmaps::Bitmap;

pub trait BitmapVecI32Conv {
    /// 取出bitmap中所有为true的索引位置
    fn bitmap_to_i32(&self) -> Vec<i32>;
}

impl BitmapVecI32Conv for Bitmap<256> {
    fn bitmap_to_i32(&self) -> Vec<i32> {
        let mut index_array = Vec::<i32>::new();

        if let Some(index) = self.first_index() {
            index_array.push(index as i32);
            let mut i = index;
            loop {
                match self.next_index(i) {
                    Some(index) => {
                        i = index;
                        index_array.push(index as i32);
                    }
                    None => break,
                }
            }
        };

        index_array
    }
}
