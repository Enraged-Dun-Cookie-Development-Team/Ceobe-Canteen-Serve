# Ceobe-Canteen-Serve

- [git emoji](https://gitmoji.dev/) 以后提交用这个吧

## 启动配置文件

启动配置文件支持 `*.toml`,`*.json`,`*.yaml` 可以将配置不同部分分布在不同的文件中（不推荐）

- 配置项介绍
  | 所属分组 | 名称 | 必须 | 类型 | 说明 | 补充 |
  | :----------------------: | :--------------------: | :---: | :-------------------------------------------------------------------------------------: | :----------------------------------- | :----------------------------------------------------------------- |
  | `database`/`db` | `scheme` | 是 | `String` | 数据库的 scheme,如 mysql | 无 |
  | `database`/`db` | `username` | 是 | `String` | 数据库的 用户名 | 无 |
  | `database`/`db` | `password` | 是 | `String` | 数据库的 密码 | 无 |
  | `database`/`db` | `host` | 否 | `String` | 数据库的 host | 默认为 `localhost` |
  | `database`/`db` | `port` | 否 | `u16` | 数据库的 port | 默认为`3306` |
  | `database`/`db` | `name` | 是 | `String` | 使用的数据库名称 | 无 |
  | `database`/`db` | `max_conn` | 是 | `u32` | 数据库的最大连接数 | 无 |
  | `database`/`db` | `min_conn` | 是 | `u32` | 数据库的最小连接数 | 无 |
  | `database`/`db` | `logger` | 否 | `bool` | 是否开始数据库操作日志 | 默认关闭 |
  | `logger`/`log` | `to_stdout` | 否 | `bool` | 同时将日志输出到 Stdout | 默认为 true |
  | `logger`/`log` | `to_file` | 否 | `String` | 同时将日志输出的文件 | 有值将会同时将日志输出到指定文件 |
  | `logger`/`log` | `level` | 是 | `off` 或者</br>`error`或者</br>`warm` 或者</br>`info` 或者</br>`debug`
  或者</br>`trace` | 日志输出过滤等级 | 无 |
  | `resp_result`/ `rresult` | `body` | 是 | `String` | 响应成功时的响应体字段名称 | 无 |
  | `resp_result`/ `rresult` | `err-msg` | 是 | `String` | 响应失败时异常字段名称 | 无 |
  | `resp_result`/ `rresult` | `fix-field` | 是 | `bool` | 响应体固定字段 | 如果无需该字段值将为`null` |
  | `resp_result`/ `rresult` | `bool-status` | 否 | `Option<String>` | 使用一个 `bool` 类型标记响应成功与否 |
  该字段缺省表示不使用 |
  | `resp_result`/ `rresult` | `body-extra-err` | 否 | `Option<String>` | 额外的异常信息在响应体中字段 |
  该字段缺省表示不使用 |
  | `resp_result`/ `rresult` | `header-extra-err` | 否 | `Option<String>` | 额外的异常信息在响应头中的字段名称 |
  该字段缺省表示不使用 |
  | `mongodb`/ `mongo` | `username` | 是 | `String` | Mongodb 进行数据库连接使用的用户 | 该用户需要完整 Admin 权限 |
  | `mongodb`/ `mongo` | `password` | 是 | `String` | Mongodb 进行数据库连接使用的用户密码 | 无 |
  | `mongodb`/ `mongo` | `host` | 否 | `String` | Mongodb 进行数据库连接使用的 host | 默认为`localhost` |
  | `mongodb`/ `mongo` | `port` | 否 | `String` | Mongodb 进行数据库连接使用的端口 | 默认为`27017` |
  | `mongodb`/ `mongo` | `db_name` | 是 | `String` | Mongodb 进行数据库连接使用的数据库 | 无 |
  | `mongodb`/ `mongo` | `query` | 否 | `HashMap<String, String>` | Mongodb 进行数据库连接使用的参数 | 默认为`{}` |
  | `user_auth`/ `auth` | `jwt`/`jwt-key` | 否 | `String` | 用户鉴权使用的`Jwt`密钥 | 最大长度不超过 32 位。过长部分将会被截断，过短部分将会被随机数填充 |
  | `user_auth`/ `auth` | `header`/`header_name` | 否 | `String` | 获取 token 的 Header | 默认为`Token` |
  | `user_auth`/ `auth` | `mob_header` | 否 | `String` | 获取 mob_id 的 Header | 默认为`mob-id` |
  | `admin_user`/ `user` | `username` | 是 | `String` | 默认后台第一个最高权限用户名 | |
  | `admin_user`/ `user` | `password` | 是 | `String` | 默认后台第一个最高权限密码 | |
  | `http_listen`/ `http` | `host` | 否 | `IpAddr` | http 监听的 host | 默认为`127.0.0.1` |
  | `http_listen`/ `http` | `port` | 否 | `u16` | http 监听的 port | 默认为`8000` |
  | `qiniu`/ `qiniu_secret` | `access_key` | 是 | `String` | 七牛云的 Access Key | |
  | `qiniu`/ `qiniu_secret` | `secret_key` | 是 | `String` | 七牛云的 Secret Key | |
  | `qiniu`/ `qiniu_secret` | `bucket` | 是 | `String` | 所有要使用的 Bucket(篮子) | 必填 |
  | `redis` | `password` | 是 | `String` | Redis 进行数据库连接使用的用户密码 | 无 |
  | `redis` | `host` | 否 | `String` | Redis 进行数据库连接使用的 host | 默认为`localhost`|
  | `redis` | `port` | 否 | `u16` | Redis 进行数据库连接使用的端口 | 默认为`6379` |
  | `redis` | `db` | 是 | `u8` | Redis 进行数据库连接使用的数据库 | 默认为`0` |
- Toml

  ```toml
  [db]
  scheme="mysql"
  username="<db_user>"
  password="<db_pwd>"

  host="localhost"
  port=3306

  name="mansion_data"
  max_conn=16
  min_conn=2

  logger=true

  [log]
  level = "debug"
  to_file = "./logout.log"
  to_stdout = true

  [rresult]
  body = "body"
  err-msg = "e-msg"
  fix-field = false
  bool-status = "is-ok"
  body-extra-err = "status"

  [mongo]
  username = "<db-user>"
  password = "<db-pwd>"
  db_name = "ceobe-canteen"
  host = "localhost"
  ```

- Yaml

  ```yaml
  log:
    to_file: ./logout.log
    level: info
  ```

- Json

  ```json
  {
    "db": {
      "password": "pwd",
      "name": "name"
    }
  }
  ```

