<script setup lang="ts">
import { ref } from 'vue'
import { useTaskStore } from '../store/task'
import { Task, get_task } from '../task/task'


var display_value = ref("")

function on_get_task(id: number) {
  get_task(id)
    .then((task: Task|null)=>{
      if (task == null) {
        display_value.value = '<<not found>>';
        console.log('not found');
      } else {
        display_value.value = task.title;
        console.log('title is {}', task.title);
      }
    })
    .catch((error)=>{console.log(error);})
}

var taskStore = useTaskStore()

</script>

<template>
  <div class="card">
    <text type="text">"{{ display_value }}"</text>
    <br>
    <input type="text" id="main_task_title" name="main_task_title" ref="main_task_title">
    <br>

    <button type="button" @click="on_get_task(Number(($refs.main_task_title as HTMLInputElement).value))">get task</button>

    <button type="button" @click="taskStore.load_tasks();">list task</button>
    <div v-for="this_task in taskStore.list_tasks" :key="this_task.id?.toString()">
      <div>
        <span>id is {{ this_task.id }}, </span>
        <span>title is {{ this_task.title }}</span>
        <span>parent is {{ this_task.parent }}</span>
      </div>
    </div>
  </div>
    <button type="button" @click="
      $router.push({name: 'create_panel'})
      ">create task</button>

</template>
