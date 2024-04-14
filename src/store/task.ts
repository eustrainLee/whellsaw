import { defineStore } from 'pinia'
import { Task, list_tasks } from '../task/task'

export interface TaskState {
  tasks: Task[]
  load_tasks(): any
  list_tasks(): any
}

export const useTaskStore = defineStore('counter', {
  state: () => {
    let tasks: Task[] = [];
    return { tasks:  tasks }
  },
  // 也可以这样定义
  // state: () => ({ count: 0 })
  actions: {
    load_tasks() {
        list_tasks()
        .then((tasks: Task[])=>{
            console.log('load tasks with state:', tasks);
            this.tasks = tasks;
        }).catch((error: any)=>{console.log(error);})
    },
  },
  getters: {
    list_tasks ():Task[] {
      return this.tasks;
    }
  }
})
