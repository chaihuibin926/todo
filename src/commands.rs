use crate::task::Task;

pub enum Command {
  Add,
  List,
  Done,
  Delete,
}

pub enum ListFilter {
  Today,
  All,
  Date(String),
}

impl Command {
  pub fn from_str(s: &str) -> Option<Command> {
    match s {
      "add" => Some(Command::Add),
      "list" => Some(Command::List),
      "done" => Some(Command::Done),
      "delete" => Some(Command::Delete),
      _ => None,
    }
  }

  pub fn parse_add_task(args: &[String]) -> Result<Task, String> {
    if args.len() < 2 {
      return Err("参数格式错误: 缺少任务内容".to_string());
    }
    let content = args[1..].join(" ");

    Ok(Task::new(content))
  }

  pub fn parse_done_task(args: &[String]) -> Result<u32, String> {
    if args.len() < 2 {
      return Err("参数格式错误: 缺少任务ID".to_string());
    }
    let id = args[1].parse::<u32>().map_err(|e| e.to_string())?;
    Ok(id)
  }

  pub fn parse_delete_task(args: &[String]) -> Result<u32, String> {
    if args.len() < 2 {
      return Err("参数格式错误: 缺少任务ID".to_string());
    }
    let id = args[1].parse::<u32>().map_err(|e| e.to_string())?;
    Ok(id)
  }

  pub fn parse_list(args: &[String]) -> Result<ListFilter, String> {
    if args.len() < 2 {
      return Ok(ListFilter::Today);
    }
    match args[1].as_str() {
      "-a" | "-all" => Ok(ListFilter::All),
      "-d" | "-date" => {
        if args[2].is_empty() {
          return Err("参数格式错误: 缺少日期".to_string());
        }
        Ok(ListFilter::Date(args[2].to_string()))
      },
      _ => Err(format!("无效的参数: {}", args[1])),
    }
  }

}