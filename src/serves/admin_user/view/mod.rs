use crate::utils::user_authorize::AuthLevel;

crate::quick_struct! {
    pub CreateUser {
        username: String
        password: String
    }

    pub UserToken {
        token: String
    }

    pub UserInfo {
        roles: [AuthLevel;1]
        name: String
    }

    pub UserName {
        username: String
    }

    pub ChangePassword {
        #[serde(rename="oldpassword")]
        old_password: String
        #[serde(rename="newpassword")]
        new_password: String
    }
}
