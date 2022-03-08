# Ceobe-Canteen-Serve

- [git emoji](https://gitmoji.dev/) 以后提交用这个吧

## 异常

- 前缀类型说明
  | 前缀 | 说明 |
  |:----: | :----|
  | `F` | 框架产生的异常 |
  | `D` | 数据库异常 |
  | `I` | IO 过程异常 |
  | `P` | 数据转换时异常 |
  | `C` | 数据校验异常 |

| 前缀 | 异常码 | httpCode | 说明                       |
| :--: | :----: | :------: | :------------------------- |
| `F`  |  0001  |   500    | `Actix` Actor MailBoxError |
| `F`  |  0002  |   500    | `Actix-web` 异常           |
| `I`  |  0001  |   500    | `std::io::Error`           |
| `P`  |  0001  |   406    | Url 转化异常               |
| `P`  |  0002  |   406    | 数字转换异常               |
| `C`  |  0001  |   406    | 范围检查未通过             |
