use crate::update_source::UploadSource;

/// 上传对象上传位置
pub trait UploadPayload {
    type Source: UploadSource + 'static;

    /// 上传位置的目录， 相同类型上传位置相同
    const DIR: &'static str;

    /// 上传对象的对象名称（Key）
    fn obj_name(&self) -> &str;

    /// 通过 [UploadLocal::DIR] 与 [UploadLocal::obj_name]
    /// 合成上传使用的完整object name
    fn full_name(&self) -> String {
        format!("{}/{}", Self::DIR, self.obj_name())
    }
}
