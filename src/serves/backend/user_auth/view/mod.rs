use orm_migrate::sql_models::admin_user::models::auth_level::AuthLevel;

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
        old_password: String
        new_password: String
    }
}
