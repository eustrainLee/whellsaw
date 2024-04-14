<script setup lang="ts">
import { ref } from 'vue'
import { TaskState, create_task, TaskConfig } from '../task/task'
import { useTaskStore } from '../store/task'

var taskStore = useTaskStore()
var task_title = ref('')
var task_state =  ref('Pending' as TaskState);
var task_parent = ref(0);

</script>

<template>
  <h2>Create Task</h2>
  <div id="input_box">
    <div>
      <label for="task_title">title</label>
      <input v-model.trim="task_title"></input>
    </div>
    <div>
      <label for="task_state">state</label>
      <select v-model="task_state">
        <option :value="'Pending'">pending</option>
        <option :value="'Doing'">doing</option>
        <option :value="'Paused'">paused</option>
        <option :value="'Canceld'">canceld</option>
        <option :value="'Done'">done</option>
        <option :value="'Failed'">failed</option>
      </select>
    </div>
    <div>
      <label for="task_parent">parent</label>
      <select v-model.number="task_parent" type="number">
        <option :value="0">none</option>
        <option v-for="task in taskStore.list_tasks" :value="task.id"> {{ task.title }} </option>
      </select>
    </div>
  </div>
  <div>
    <button type="button" @click="
      create_task({
        title: task_title,
        state: task_state as TaskState,
        parent: task_parent,
      } as TaskConfig).then((id: number)=> {
        console.log(id);
        taskStore.load_tasks();
      })
      .catch((error: any)=>{console.log(error);});
    ">create</button>
    <button type="button" @click="$router.back()">back</button>
  </div>
</template>
<style>
#input_box {
  /* place-content: flex-start center; */
  /* background-color: rgba(214, 187, 240, 0.158); */
  display: flexbox;
  width: 240px;
  /* display: flex; */
}

#input_box div {
  margin-top: 10px;
  height: 30px;
}

#input_box div label {
  height: 24px;
  width: 60px;
  margin:0;
  border: 0;
  float:left;
  text-align: right;
  border: 0;
  /* background-color: yellowgreen; */
}

#input_box div select, #input_box div input {
  height: 24px;
  width: 160px;
  text-align: left;
  float: right;
  padding: 0;
  margin:0;
  border: 0;
  
  /* background-color: saddlebrown; */
}

</style>