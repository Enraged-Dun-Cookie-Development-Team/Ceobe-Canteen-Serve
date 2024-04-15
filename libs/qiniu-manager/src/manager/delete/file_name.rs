pub struct FileName<'u, 's, const BATCH_SIZE: usize> {
    pub names: &'u [&'s str],
}

impl<'u, 's, const BATCH_SIZE: usize> FileName<'u, 's, BATCH_SIZE> {
    pub fn new(all_files: &mut &'u [&'s str]) -> Option<Self> {
        match all_files.len() {
            // 用户数量为0 None
            0 => None,
            // 用户小于一个batch size 全部带走
            len if len > 0 && len <= BATCH_SIZE => {
                let names = &all_files[0..];
                *all_files = &[];
                Some(Self { names })
            }
            // 用户总数量大于一个Batch size ， 满
            _ => {
                let names = &all_files[0..BATCH_SIZE];
                *all_files = &all_files[BATCH_SIZE..];
                Some(Self { names })
            }
        }
    }
}
