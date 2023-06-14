<template>
    <div class="about">
      <h1>This is an about page</h1>
    </div>
    <div>
        <button @click="clk($event)">Click</button>
    </div>
</template>
  
<script setup lang="ts">

import init, {greet} from '../../wworker/pkg'
const wasm = await init()

console.log("ABOUT:SETUP",wasm)



function clk(evt: Event) {
    console.log("Click", evt)
    greet()
}
</script>

  <style>
  @media (min-width: 1024px) {
    .about {
      min-height: 2vh;
      display: flex;
      align-items: center;
    }
  }
</style>
  