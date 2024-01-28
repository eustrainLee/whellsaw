<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api'

defineProps<{ msg: string }>()

var new_id = ref(0)

// not used
// interface Task {
//   id?: number | null,
//   title: string,
//   create_time?: Date | null | undefined,
//   last_update_time?: Date | null | undefined,
//   childs: [number] | [],
//   state: "Pending" | "Doing" | "Paused" | "Canceld" | "Done" | "Failed"
// }

interface TaskConfig {
  title: string
}

async function doClickCount(count :number) :Promise<number> {
    return await invoke("next_number", { value: count })
}

async function newTask(title: string) :Promise<number> {
    console.log("title is ", title)
    return await invoke("new_task", {
      t: {
        title: title,
      } as TaskConfig,
    })
}
</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <button type="button" @click="doClickCount(new_id).then((next)=>new_id=next)">id is {{ new_id }}</button>
    <br>
    <input type="text" id="task_title" name="task_title" ref="task_title">
    <br>
    <button type="button" @click="newTask(($refs.task_title as HTMLInputElement).value)
      .then((id: number)=>{
            new_id=id;
            console.log('id is {}', id);
        })
      .catch((error)=>{console.log(error)}
      )">new task</button>
  </div>

</template>
