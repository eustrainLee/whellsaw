<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api'

defineProps<{ msg: string }>()

var count = ref(0)


async function doClickCount(count :number) :Promise<number> {
    return await invoke("next_number", { value: count })
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
  </div>

</template>
