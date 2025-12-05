yoyo# Todo - 命令行任务管理工具

一个简洁的命令行任务管理工具，使用 Rust 编写。

## 安装

```bash
cargo build --release
cargo install --path .
```

## 可用命令

```bash
# 添加任务
todo add <任务内容>

# 列出今天的任务（默认）
todo list

# 列出所有任务
todo list -a

# 列出指定日期的任务
todo list -d <日期>  # 格式: YYYY-MM-DD

# 标记任务为已完成
todo done <任务ID>

# 删除任务
todo delete <任务ID>
```

## 示例

```bash
todo add 完成项目文档
todo list
todo list -a
todo list -d 2024-01-15
todo done 1
todo delete 1
```