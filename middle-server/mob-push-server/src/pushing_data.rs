use std::borrow::Cow;

use crate::{
    push_notify::{android::AndroidNotify, ios::IosNotify},
    PushForward,
};

/// the trait of Entity for Push
pub trait PushEntity: 'static + Sync + Send {
    /// 获取当前推送消息的推送正文
    type Content: AsRef<str> + 'static + Sync + ?Sized;

    fn get_send_content(&self) -> &Self::Content;

    /// 获取当前推送消息的标题
    fn get_title(&self) -> Cow<'_, str> { "新饼来袭".into() }
    /// 获取当前推送消息的安卓端配置
    fn android_notify(&self, _notify: &mut AndroidNotify) {}

    /// 获取当前推送消息的Ios端配置
    fn ios_notify(&self, _notify: &mut IosNotify) {}

    fn push_forward(&self, _push_forward: &mut PushForward) {}
}
