<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api'

defineProps<{ msg: string }>()

var count = ref(0)
var str_value = ref("")


async function doClickCount(count :number) :Promise<number> {
    return await invoke("next_number", { value: count })
}

async function doRefresh(current: string, count: number) :Promise<[string, number]> {
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
