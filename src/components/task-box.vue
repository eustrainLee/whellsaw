<script setup lang="ts">
import { ref } from 'vue'
import { Task, createTask, getTask, listTask } from '../task/task'

defineProps<{ msg: string }>()

var display_value = ref("")
var total_tasks = ref([] as Task[]);

function on_get_task(id: number) {
  getTask(id)
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

</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <text type="text">"{{ display_value }}"</text>
    <br>
    <input type="text" id="task_title" name="task_title" ref="task_title">
    <br>
    <button type="button" @click="createTask(($refs.task_title as HTMLInputElement).value)
      .then((id: number)=>{
            display_value=String(id);
            console.log('id is {}', id);
        })
      .catch((error)=>{console.log(error);})
      ">create task</button>

    <button type="button" @click="on_get_task(Number(($refs.task_title as HTMLInputElement).value))">get task</button>

    <button type="button" @click="listTask()
      .then((tasks: [Task])=>{
        display_value = '';
        console.log('tasks:');
        for (let i = 0; i < tasks.length; i++) {
          display_value = display_value + tasks[i].id + ':' + tasks[i].title + ';'
          console.log('id is {}, title is {}', tasks[i].id, tasks[i].title);
        }
        total_tasks = tasks;
      })
      .catch((error)=>{console.log(error);})
      ">list task</button>
      <div v-for="this_task in total_tasks" :key="this_task.id?.toString()">
        <div>
          <span>id is {{ this_task.id }},</span>
          <span>title is {{ this_task.title }}</span>
        </div>
      </div>
  </div>

</template>
