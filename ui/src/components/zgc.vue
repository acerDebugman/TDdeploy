<!-- <script setup>
import { ref } from 'vue'

const count = ref(0)

function increment() {
  count.value++
}
</script> -->
<script setup>
import { ref, computed } from 'vue'

const count = ref(2)

// 这个计算属性在 count 的值小于或等于 3 时，将返回 count 的值。
// 当 count 的值大于等于 4 时，将会返回满足我们条件的最后一个值
// 直到 count 的值再次小于或等于 3 为止。
// const alwaysSmall = computed((previous) => {
//   if (count.value <= 3) {
//     return count.value
//   }

//   return previous
// })
const alwaysSmall = computed({
  get(previous) {
    if (count.value <= 3) {
      return count.value
    }

    return previous
  },
  set(newValue) {
    count.value = newValue * 2
  }
})

function increment() {
    count.value++
}
function desc() {
    count.value--
}
function test_set() {
    alwaysSmall.value = 3
}

let todos = ref([
  { name: '学习 Vue', completed: false },
  { name: '学习 React', completed: true },
  { name: '学习 Angular', completed: false }
])

function reorder_todos() {
    todos.value = todos.value.reverse()
}

function warn(message, event) {
  // 这里可以访问原生事件
  if (event) {
    event.preventDefault()
  }
  alert(message)
}

let toggle = ref('')
</script>

<template>
  <button @click="increment">
    xxx1: {{ count }}
  </button>
  <button @click="increment">
    incr2: {{ alwaysSmall }}
  </button>
  <button @click="desc">
    desc3: {{ alwaysSmall }}
  </button>
  <button @click="test_set">
    set=3: {{ alwaysSmall }}
  </button>
  <button @click="reorder_todos">
    reorder: {{ todos }}
  </button>
  <div width="800px">
    <template v-for="todo in todos" :key="todo.name">
      <li>{{ todo.name }}</li>
    </template>
  </div>
  <button @click="warn('Form cannot be submitted yet.', $event)">
    Submit
  </button>

  <p>toggle is: {{ toggle }}</p>
  <input
  type="checkbox"
  v-model="toggle"
  true-value="yes"
  false-value="no" />
</template>