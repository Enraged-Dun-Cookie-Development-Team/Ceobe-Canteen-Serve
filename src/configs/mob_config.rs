use mob_push_server::MobPushConfigTrait;


crate::quick_struct! {
    pub MobPushConfig {
        app_key: String
        app_secret: String
    }
}

impl MobPushConfigTrait for MobPushConfig {
    fn get_key(&self) -> &str {
        &self.app_key
    }

    fn get_secret(&self) -> &str {
        &self.app_secret
    }
}