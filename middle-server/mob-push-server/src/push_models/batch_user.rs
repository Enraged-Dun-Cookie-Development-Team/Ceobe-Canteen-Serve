use serde::{ser::SerializeStruct, Serialize};

pub struct BatchUsers<'u, 's, const BATCH_SIZE: usize> {
    pub users: &'u [&'s str],
}

impl<'u, 's, const BATCH_SIZE: usize> BatchUsers<'u, 's, BATCH_SIZE> {
    pub fn new(all_users: &mut &'u [&'s str]) -> Option<Self> {
        match all_users.len() {
            // 用户数量为0 None
            0 => None,
            // 用户小于一个batch size 全部带走
            len if len > 0 && len <= BATCH_SIZE => {
                let users = &all_users[0..];
                *all_users = &[];
                Some(Self { users })
            }
            // 用户总数量大于一个Batch size ， 满
            _ => {
                let users = &all_users[0..BATCH_SIZE];
                *all_users = &all_users[BATCH_SIZE..];
                Some(Self { users })
            }
        }
    }
}

impl<'u, 's, const BATCH_SIZE: usize> Serialize
    for BatchUsers<'u, 's, BATCH_SIZE>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut push_target = serializer.serialize_struct("pushTarget", 2)?;

        push_target.serialize_field("target", &4)?;
        push_target.serialize_field("rids", &self.users)?;

        push_target.end()
    }
}
