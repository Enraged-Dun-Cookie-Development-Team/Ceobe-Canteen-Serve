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
  | `logger`/`log` | `logger_target` | 是 | `file`或者`stdout` | 日志输出目的 | 无 |
  | `logger`/`log` | `to_file` | 是 | `String` | 日志输出的文件 | `logger_target` 为 `file` 时才有效 |
  | `logger`/`log` | `enable_color` | 否 | `bool` | 日志输出是否带颜色 | `logger_target` 为 `stdout` 时才有效,默认为 `true` |
  | `logger`/`log` | `level` | 是 | `off` 或者</br>`error`或者</br>`warm` 或者</br>`info` 或者</br>`debug` 或者</br>`trace` | 日志输出过滤等级 | 无 |
  | `resp_result`/ `rresult` | `body` | 是 | `String` | 响应成功时的响应体字段名称 | 无 |
  | `resp_result`/ `rresult` | `err-msg` | 是 | `String` | 响应失败时异常字段名称 | 无 |
  | `resp_result`/ `rresult` | `fix-field` | 是 | `bool` | 响应体固定字段 | 如果无需该字段值将为`null` |
  | `resp_result`/ `rresult` | `bool-status` | 否 | `Option<String>` | 使用一个 `bool` 类型标记响应成功与否 | 该字段缺省表示不使用 |
  | `resp_result`/ `rresult` | `body-extra-err` | 否 | `Option<String>` | 额外的异常信息在响应体中字段 | 该字段缺省表示不使用 |
  | `resp_result`/ `rresult` | `header-extra-err` | 否 | `Option<String>` | 额外的异常信息在响应头中的字段名称 | 该字段缺省表示不使用 |
  | `mongodb`/ `mongo` | `username` | 是 | `String` | Mongodb 进行数据库连接使用的用户 | 该用户需要完整 Admin 权限 |
  | `mongodb`/ `mongo` | `password` | 是 | `String` | Mongodb 进行数据库连接使用的用户密码 | 无 |
  | `mongodb`/ `mongo` | `host` | 否 | `String` | Mongodb 进行数据库连接使用的 host | 默认为`localhost` |
  | `mongodb`/ `mongo` | `port` | 否 | `String` | Mongodb 进行数据库连接使用的端口 | 默认为`27017` |
  | `mongodb`/ `mongo` | `db_name` | 是 | `String` | Mongodb 进行数据库连接使用的数据库 | 无 |
  | `user_auth`/ `auth` | `jwt`/`jwt-key` | 否 | `String` | 用户鉴权使用的`Jwt`密钥 | 最大长度不超过 32 位。过长部分将会被截断，过短部分将会被随机数填充 |
  | `user_auth`/ `auth` | `header`/`header_name` | 否 | `String` | 获取 token 的 Header | 默认为`Token` |
  | `admin_user`/ `user` | `username` | 是 | `String` | 默认后台第一个最高权限用户名 | |
  | `admin_user`/ `user` | `password` | 是 | `String` | 默认后台第一个最高权限密码 | |
  | `http_listen`/ `http` | `host` | 否 | `IpAddr` | http 监听的 host | 默认为`127.0.0.1` |
  | `http_listen`/ `http` | `port` | 否 | `u16` | http 监听的 port | 默认为`8000` |

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
  logger_target="stdout"

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

## 异常

- 前缀类型说明
  | 前缀 | 说明 |
  | :---: | :------------- |
  | `F`| 服务器异常 |
  | `D` | 数据库异常 |
  | `G` | MongoDb 异常 |
  | `I` | IO 过程异常 |
  | `P` | 数据转换时异常 |
  | `C` | 数据校验异常 |
  | `S` | 资源查找异常 |
  | `M` | 数据未变更 |
  | `A` | 权限认证异常 |

- 服务异常

  | 前缀 | 异常码 | httpCode | 说明                   |
  | :--: | :----: | :------: | :--------------------- |
  | `F`  |  0001  |   500    | 服务器发生未预期 Panic |

- Io 异常

  | 前缀 | 异常码 | httpCode | 说明             |
  | :--: | :----: | :------: | :--------------- |
  | `I`  |  0001  |   500    | `std::io::Error` |

- 类型转换异常

  | 前缀 | 异常码 | httpCode | 说明           |
  | :--: | :----: | :------: | :------------- |
  | `P`  |  0001  |   406    | Url 转化异常   |
  | `P`  |  0002  |   406    | 数字转换异常   |
  | `P`  |  0003  |   406    | jwt 转换异常   |
  | `P`  |  0004  |   406    | 日期转换异常   |
  | `P`  |  0004  |   500    | 字符串编码异常 |

- 数据校验异常

  | 前缀 | 异常码 | httpCode | 说明                             |
  | :--: | :----: | :------: | :------------------------------- |
  | `C`  |  0001  |   406    | 范围检查未通过                   |
  | `C`  |  0002  |   406    | 饼学大厦 id 格式不是 {int}.{int} |
  | `C`  |  0003  |   406    | 错误的 Fraction 值范围(0~5)      |
  | `C`  |  0004  |   406    | `Json` 序列化/反序列化异常       |
  | `C`  |  0005  |   406    | `Path` 数据加载异常              |
  | `C`  |  0006  |   406    | 未知的预期确信度等级             |
  | `C`  |  0007  |   406    | `Query` 加载异常                 |
  | `C`  |  0008  |   409    | 饼学大厦的 Id 已经存在           |
  | `C`  |  0009  |   406    | BV 号格式错误                    |

- 数据库异常（SeaOrm）

  | 前缀 | 异常码 | httpCode | 说明                 |
  | :--: | :----: | :------: | :------------------- |
  | `D`  |  0001  |   500    | 数据库连接失败       |
  | `D`  |  0002  |   500    | 数据库请求操作失败   |
  | `D`  |  0003  |   500    | 数据库查询失败       |
  | `D`  |  0004  |   500    | 数据库记录不存在     |
  | `D`  |  0005  |   500    | 数据库 `CustomError` |
  | `D`  |  0006  |   500    | 数据库类型转换失败   |
  | `D`  |  0007  |   500    | 数据库类型序列化失败 |
  | `D`  |  0008  |   500    | 数据库 Migrate 失败  |

- 数据库异常（MongoDb）

  | 前缀 | 异常码 | httpCode | 说明                     |
  | :--: | :----: | :------: | :----------------------- |
  | `G`  |  0001  |   500    | 非法参数                 |
  | `G`  |  0002  |   500    | 权限不足                 |
  | `G`  |  0003  |   500    | `Bson`反序列化失败       |
  | `G`  |  0004  |   500    | `Bson`序列化失败         |
  | `G`  |  0005  |   500    | `Bson`序列化失败         |
  | `G`  |  0006  |   500    | 写冲突                   |
  | `G`  |  0007  |   500    | 指令错误                 |
  | `G`  |  0008  |   500    | `DNS`处理异常            |
  | `G`  |  0009  |   500    | 网络异常                 |
  | `G`  |  0010  |   500    | Io 异常                  |
  | `G`  |  0011  |   500    | 非法响应                 |
  | `G`  |  0012  |   500    | 客户端无法选择数据库服务 |
  | `G`  |  0013  |   500    | 客户端未提供 Session     |
  | `G`  |  0014  |   500    | 非法 TLS 配置            |
  | `G`  |  0015  |   500    | 写入异常                 |
  | `G`  |  0016  |   500    | 事务异常                 |
  | `G`  |  0017  |   500    | 服务不可用异常           |
  | `G`  |  0018  |   500    | 数据库不存在             |
  | `G`  |  0019  |   500    | 数据库中集合不存在       |

- 数据未变更

  | 前缀 | 异常码 | httpCode | 说明                           |
  | :--: | :----: | :------: | :----------------------------- |
  | `M`  |  0001  |   304    | Ceobe 在当前提供时间戳下无更新 |

- 资源查找异常

  | 前缀 | 异常码 | httpCode | 说明               |
  | :--: | :----: | :------: | :----------------- |
  | `S`  |  0001  |   404    | 指定饼学大厦未找到 |
  | `S`  |  0002  |   404    | 不存在的路由       |
  | `S`  |  0003  |   500    | 发起请求时出现异常       |

- 权限认证异常

  | 前缀 | 异常码 | httpCode | 说明                 |
  | :--: | :----: | :------: | :------------------- |
  | `A`  |  0001  |   401    | 缺少 Token 字段      |
  | `A`  |  0002  |   401    | 权限不足             |
  | `A`  |  0003  |   404    | Token 对应信息不存在 |
  | `A`  |  0004  |   401    | 密码错误             |
  | `A`  |  0005  |   500    | 密码处理错误         |
  | `A`  |  0006  |   401    | Token 失效           |
  | `A`  |  0007  |   404    | 用户不存在           |
  | `A`  |  0008  |   400    | 用户已经被使用       |
  | `A`  |  0009  |   400    | 密码未更改           |
