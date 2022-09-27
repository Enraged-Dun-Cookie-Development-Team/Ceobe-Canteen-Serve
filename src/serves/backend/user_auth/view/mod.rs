use orm_migrate::sql_models::admin_user::models::{
    auth_level::AuthLevel, user::UserList,
};

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

    pub UserTable {
        id: i32
        username: String
        auth:AuthLevel
    }

    pub PageSize {
        page: u64
        size: u64
        total_count: u64
        total_page: u64
    }

    pub ViewUserListResq {
        user_table: Vec<UserTable>
        page_size: PageSize
    }

    pub UserName {
        username: String
    }

    pub ChangePassword {
        old_password: String
        new_password: String
    }

    pub ChangeAuthReq {
        id: i32
        auth: AuthLevel
    }

    pub DeleteOneUserReq {
        id: i32
    }

    pub UserListReq {
        page: u64
        size: u64
    }
}

impl From<UserList> for UserTable {
    fn from(UserList { username, auth, id }: UserList) -> Self {
        Self { id, username, auth }
    }
}
