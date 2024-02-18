<script setup lang="ts">
import { ref } from 'vue'
import { Task, newTask, getTask, listTask } from '../task/task'

defineProps<{ msg: string }>()

var display_value = ref("")
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
      .catch((error)=>{console.log(error);})
      ">new task</button>

    <button type="button" @click="getTask(Number(($refs.task_title as HTMLInputElement).value))
      .then((task: Task|null)=>{
        if (task == null) {
          display_value = '<<not found>>';
          console.log('not found');
        } else {
          display_value = task.title;
          console.log('title is {}', task.title);
        }
      })
      .catch((error)=>{console.log(error);})
      ">get task</button>

    <button type="button" @click="listTask()
      .then((tasks: [Task])=>{
        display_value = '';
        console.log('tasks:');
        for (let i = 0; i < tasks.length; i++) {
          display_value = display_value + tasks[i].id + ':' + tasks[i].title + ';'
          console.log('id is {}, title is {}', tasks[i].id, tasks[i].title);
        }
      })
      .catch((error)=>{console.log(error);})
      ">list task</button>
  </div>

</template>
