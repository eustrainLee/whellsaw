import { invoke } from '@tauri-apps/api'

export interface Task {
    id?: number | null,
    title: string,
    create_time?: Date | null | undefined,
    last_update_time?: Date | null | undefined,
    childs: [number] | [],
    state: "Pending" | "Doing" | "Paused" | "Canceld" | "Done" | "Failed"
  }
  
  export interface TaskConfig {
    title: string
  }
  
  export async function newTask(title: string) :Promise<number> {
      console.log("title is ", title)
      return await invoke("new_task", {
        t: {
          title: title,
        } as TaskConfig,
      })
  }
  
  export async function getTask(id: number) :Promise<Task|null> {
      return await invoke("get_task", {
        id: id,
      })
  }
  
  export async function listTask() :Promise<[Task]> {
      return await invoke("list_task", {})
  }