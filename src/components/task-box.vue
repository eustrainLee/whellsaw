<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api'

defineProps<{ msg: string }>()

var count = ref(0)
var str_value = ref("")

interface Task {
  id?: number | null,
  title: string,
  create_time?: Date | null | undefined,
  last_update_time?: Date | null | undefined,
  childs: [number] | [],
  state: "Pending" | "Doing" | "Paused" | "Canceld" | "Done" | "Failed"
}

async function doClickCount(count :number) :Promise<number> {
    return await invoke("next_number", { value: count })
}

async function doRefresh(current: string, count: number) :Promise<[string, number]> {
    await invoke("new_task", {
      t: {
        id: 1,
        title: "task_name",
        // create_time: null,
        // last_update_time: Date(),
        childs: [],
        state: "Pending"
      } as Task,
    })
    return await invoke("refresh", { value: current, count: count })
}
</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <button type="button" @click="doClickCount(count).then((next)=>count=next)">count is {{ count }}</button>
    <p>
      Edit
      <code>components/task-box.vue</code> to test HMR
    </p>
    <p>value is "{{ str_value }}"</p>
    <button type="button" @click="doRefresh(str_value, count).then((tuple)=>{str_value=tuple[0]; count=tuple[1];})">reflush</button>
  </div>

</template>
