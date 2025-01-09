# 教程
## 实体生成数据表
### 如果安装了就跳过
`` cargo install sea-orm-cli ``

### 初始化
 ``sea-orm-cli migrate init ``

### 创建表
 ``sea-orm-cli migrate up``

### 生成实体
 ``sea-orm-cli generate entity  -o entity/src ``

 ### build
 `` CROSS_COMPILE=x86_64-linux-musl- cargo build --release --target x86_64-unknown-linux-musl ``
 or

 `` cargo build --release --target x86_64-unknown-linux-musl ``

 參考：[编译为 linux x86 目标](https://blog.csdn.net/jiaoyangwm/article/details/136367639)