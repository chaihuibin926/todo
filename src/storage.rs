use crate::task::Task;
use std::fs;
use std::path::{PathBuf};

fn get_data_file_path() -> PathBuf {
  let home_dir = dirs::home_dir().expect("获取用户主目录失败");
  let data_dir = home_dir.join(".todo");

  if !data_dir.exists() {
    fs::create_dir_all(&data_dir).expect("创建数据目录失败");
  }

  data_dir.join("todo_list.json")
}

pub fn insert_task(mut task: Task) -> Result<(), String> {
  let mut tasks = query_tasks()?;

  task.id = tasks.len() as u32 + 1;
  tasks.push(task);
  
  write_tasks(tasks);

  Ok(())
}

fn write_tasks(tasks: Vec<Task>) {
  let file_path = get_data_file_path();
  let _ = fs::write(
    file_path, 
    serde_json::to_string(&tasks).unwrap()
  );
}

pub fn query_tasks() -> Result<Vec<Task>, String> {
  let file_path = get_data_file_path();
  if file_path.exists() {
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