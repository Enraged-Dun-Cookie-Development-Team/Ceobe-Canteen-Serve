use crate::user_create::default_user::FUserConfig;

crate::quick_struct! {
    pub FirstUserConfig {
        username:String
        password:String
    }
}

impl FUserConfig for FirstUserConfig {
    fn username(&self) -> String {
        self.username.clone()
    }

    fn password(&self) -> String {
        self.password.clone()
    }
}

