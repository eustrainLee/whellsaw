<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api'

defineProps<{ msg: string }>()

var display_value = ref("")

// not used
interface Task {
  id?: number | null,
  title: string,
  create_time?: Date | null | undefined,
  last_update_time?: Date | null | undefined,
  childs: [number] | [],
  state: "Pending" | "Doing" | "Paused" | "Canceld" | "Done" | "Failed"
}

interface TaskConfig {
  title: string
}
async function newTask(title: string) :Promise<number> {
    console.log("title is ", title)
    return await invoke("new_task", {
      t: {
        title: title,
      } as TaskConfig,
    })
}

async function getTask(id: number) :Promise<Task> {
    return await invoke("get_task", {
      id: id,
    })
}

async function listTask() :Promise<[Task]> {
    return await invoke("list_task", {})
}
</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <text type="text">"{{ display_value }}"</text>
    <br>
    <input type="text" id="task_title" name="task_title" ref="task_title">
    <br>
    <button type="button" @click="newTask(($refs.task_title as HTMLInputElement).value)
      .then((id: number)=>{
            display_value=String(id);
            console.log('id is {}', id);
        })
      .catch((error)=>{console.log(error)}
      )">new task</button>

    <button type="button" @click="getTask(Number(($refs.task_title as HTMLInputElement).value))
      .then((task: Task)=>{
        display_value = task.title
            console.log('title is {}', task.title);
        })
      .catch((error)=>{console.log(error)}
      )">get task</button>

    <button type="button" @click="listTask()
      .then((tasks: [Task])=>{
        console.log('tasks:');
        display_value = '';
        for (let i = 0; i < tasks.length; i++) {
          display_value = display_value + tasks[i].id + ':' + tasks[i].title + ';'
          console.log('id is {}, title is {}', tasks[i].id, tasks[i].title);
        }
        })
      .catch((error)=>{console.log(error)}
      )">list task</button>
  </div>

</template>
