<script setup lang="ts">
import { create_task, TaskConfig } from '../task/task'
import { useTaskStore } from '../store/task'

var taskStore = useTaskStore()
</script>

<template>
  <h2>Create Task</h2>
  <div id="input_box">
    <div>
      <label for="task_title">title</label>
      <input id="task_title" ref="task_title"></input>
    </div>
    <div>
      <label for="test_state">state</label>
      <select id="task_state" ref="task_state">
        <option :key="'Pending'" :value="'Pending'">pending</option>
        <option :key="'Doing'" :value="'Doing'">doing</option>
        <option :key="'Paused'" :value="'Paused'">paused</option>
        <option :key="'Canceld'" :value="'Canceld'">canceld</option>
        <option :key="'Done'" :value="'Done'">done</option>
        <option :key="'Failed'" :value="'Failed'">failed</option>
      </select>
    </div>
    <div>
      <label for="task_parent">parent</label>
      <select id="task_parent" ref="task_parent">
        <option :key="0" :value="0"></option>
        <option v-for="task in taskStore.list_tasks" :key="task.id" :value="task.id"> {{ task.title }} </option>
      </select>
    </div>
  </div>
  <div>
    <button type="button" @click="
      console.log(($refs.task_title as HTMLInputElement).value);
      console.log(($refs.task_state as HTMLInputElement).value);
      console.log(($refs.task_parent as HTMLInputElement).value);
      create_task({
        title: ($refs.task_title as HTMLInputElement).value as string,
        state: ($refs.task_state as HTMLInputElement).value as string,
        parent: +(($refs.task_parent as HTMLInputElement).value as string) as number
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