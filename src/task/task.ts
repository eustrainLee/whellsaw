import { invoke } from '@tauri-apps/api'

export interface Task {
    id: number,
    title: string,
    create_time?: Date | null,
    last_update_time?: Date | null,
    parent: number,
    childs: [number] | [],
    state: "Pending" | "Doing" | "Paused" | "Canceld" | "Done" | "Failed"
  }
  
  export interface TaskConfig {
    title: string,
    state: "Pending" | "Doing" | "Paused" | "Canceld" | "Done" | "Failed"
    parent: number
  }
  
  export async function create_task_with_title(title: string) :Promise<number> {
      console.log("title is ", title)
      return await invoke("create_task", {
        t: {
          title: title,
        } as TaskConfig,
      })
  }
  export async function create_task(task_cfg: TaskConfig) :Promise<number> {
      console.log("create task with cfg:", task_cfg)
      return await invoke("create_task", {t: task_cfg})
  }
  
  export async function get_task(id: number) :Promise<Task|null> {
      return await invoke("get_task", {
        id: id,
      })
  }
  
  export async function list_tasks() :Promise<[Task]> {
      return await invoke("list_task", {})
  }