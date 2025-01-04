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