use bitmaps::Bitmap;

pub trait BitmapVecUsizeConv {
    /// 取出bitmap中所有为true的索引位置
    fn bitmap_to_usize(&self) -> Vec<usize>;
}

impl BitmapVecUsizeConv for Bitmap<256> {
    fn bitmap_to_usize(&self) -> Vec<usize> {
        let mut index_array = Vec::<usize>::new();

        if let Some(index) = self.first_index() {
            index_array.push(index);
            let mut i = index;
            while let Some(index) = self.next_index(i) {
                i = index;
                index_array.push(index);
            }
        };

        index_array
    }
}
