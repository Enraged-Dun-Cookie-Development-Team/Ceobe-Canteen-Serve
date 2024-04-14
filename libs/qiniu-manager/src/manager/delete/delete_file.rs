use qiniu_objects_manager::{BatchOperations, BatchOperationsStream, Bucket};

use super::file_name::FileName;




pub struct DeleteIter<
    'file,
    'string,
    const BATCH_SIZE: usize,
> {
    pub files_names: &'file [&'string str],
    pub bucket: &'file Bucket,
}

impl <
    'file,
    'string,
    const BATCH_SIZE: usize,
> Iterator for DeleteIter<'file, 'string, BATCH_SIZE> {
    type Item = BatchOperationsStream<'file>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(files) = FileName::<'file, 'string, BATCH_SIZE>::new(&mut self.files_names)
        else {
            return None;
        };

        Some(files.names.iter().fold(self.bucket.batch_ops(), |mut ops, obj| {
            ops.add_operation(self.bucket.delete_object(obj));
            ops
        })
        .async_call())
    }
}