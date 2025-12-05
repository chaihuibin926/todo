use crate::task::Task;
use std::fs;
use std::path::Path;

pub fn insert_task(mut task: Task) -> Result<(), String> {
  let mut tasks = query_tasks()?;

  task.id = tasks.len() as u32 + 1;
  tasks.push(task);
  
  write_tasks(tasks);

  Ok(())
}

fn write_tasks(tasks: Vec<Task>) {
  let file_path = "todo_list.json";
  let _ = fs::write(
    file_path, 
    serde_json::to_string(&tasks).unwrap()
  );
}

pub fn query_tasks() -> Result<Vec<Task>, String> {
  let file_path = "todo_list.json";
  if Path::new(file_path).exists() {
    let content = fs::read_to_string(file_path)
      .map_err(|e| format!("读取文件失败: {}", e))?;

    if content.trim().is_empty() {
      Ok(Vec::new())
    } else {
      let tasks: Vec<Task> = serde_json::from_str(&content)
        .map_err(|e| format!("解析文件失败: {}", e))?;
      
      Ok(tasks)
    }
  } else {
    Ok(Vec::new())
  }
}

pub fn done_task(id: u32) -> Result<(), String> {
  let mut tasks = query_tasks()?;

  tasks.iter_mut()
    .find(|task| task.id == id)
    .map(|task| task.done = true);
  
  write_tasks(tasks);

  Ok(())
}

pub fn delete_task(id: u32) -> Result<(), String> {
  let mut tasks = query_tasks()?;
  tasks.retain(|task| task.id != id);
  write_tasks(tasks);
  Ok(())
}