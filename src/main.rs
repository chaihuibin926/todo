mod commands;
mod task;
mod storage;

use std::env;
use commands::{Command, ListFilter};
use storage::{insert_task, query_tasks, done_task, delete_task};
use chrono::{Local, TimeZone, NaiveDate};

fn print_help() {
    println!("Todo 任务管理工具");
    println!();
    println!("用法: todo <命令> [参数]");
    println!();
    println!("可用命令:");
    println!("  add <任务内容>    添加新任务");
    println!("  list             列出今天的任务（默认）");
    println!("  list -a           列出所有任务");
    println!("  list -d <日期>    列出指定日期的任务（格式: YYYY-MM-DD）");
    println!("  done <任务ID>     标记任务为已完成");
    println!("  delete <任务ID>   删除任务");
    println!();
    println!("示例:");
    println!("  todo add 完成项目文档");
    println!("  todo list");
    println!("  todo list -a");
    println!("  todo list -d 2024-01-15");
    println!("  todo done 1");
    println!("  todo delete 1");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let method = &args[1];
    let args = &args[1..];

    match Command::from_str(method) {
        Some(Command::Add) => {
            let task = Command::parse_add_task(&args).unwrap();
            println!("添加任务: {}", task);
            let _ = insert_task(task).map_err(|e| println!("{}", e));
        }
        Some(Command::List) => {
            let filter = Command::parse_list(&args).unwrap();

            let tasks = query_tasks()
                .map_err(|e| println!("{}", e));

            match filter {
                ListFilter::Today => {
                    for task in tasks.unwrap() {
                        let task_date = Local.timestamp_opt(task.created_at, 0).unwrap();
                        if task_date.date_naive() == Local::now().date_naive() {
                            println!("{}", task);
                        }
                    }
                },
                ListFilter::All => {
                    for task in tasks.unwrap() {
                        println!("{} ---- {}", task, task.date().format("%Y-%m-%d"));
                    }
                },
                ListFilter::Date(date) => {
                    for task in tasks.unwrap() {
                        let task_date = Local.timestamp_opt(task.created_at, 0).unwrap();
                        if task_date.date_naive() == NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap() {
                            println!("{}", task);
                        }
                    }
                },
            }
        }
        Some(Command::Done) => {
            let task_id = Command::parse_done_task(&args).unwrap();
            println!("完成任务: {}", task_id);
            let _ = done_task(task_id).map_err(|e| println!("{}", e));
        }
        Some(Command::Delete) => {
            let task_id = Command::parse_delete_task(&args).unwrap();
            println!("删除任务: {}", task_id);
            let _ = delete_task(task_id).map_err(|e| println!("{}", e));
        }
        None => {
            println!("无效命令");
        }
    }
}
