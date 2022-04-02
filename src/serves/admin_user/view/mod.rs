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
        oldpassword: String
        newpassword: String
    }
}
