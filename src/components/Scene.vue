<template>
  <canvas
      ref="canvas"
      class="skybox"
      @mousedown="handleMouseDown"
      @wheel="handleWheel"
  ></canvas>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import init, { initialize, render, update_camera, zoom_camera } from '../pkg/black_hole_wasm.js'

const props = defineProps({
  width: { type: Number, default: 800 },
  height: { type: Number, default: 600 },
  sensitivity: { type: Number, default: 0.15 }
})

const canvas = ref<HTMLCanvasElement | null>(null)
let isDragging = false
let lastX = 0
let lastY = 0

// 初始化 WASM
const setupWasm = async () => {
  await init()        // 初始化 WASM
  initialize()        // 调用 Rust 端的初始化（会加载纹理和设置摄像机）
  startRenderLoop()   // 开始渲染循环
}

// 渲染循环
const startRenderLoop = () => {
  const ctx = canvas.value!.getContext('2d')!
  const animate = () => {
    render(ctx)       // 调用 Rust 渲染（直接从 Rust 渲染到 canvas）
    requestAnimationFrame(animate)  // 保持动画循环
  }
  requestAnimationFrame(animate)
}

// 鼠标按下：开始拖拽
const handleMouseDown = (e: MouseEvent) => {
  isDragging = true
  lastX = e.clientX
  lastY = e.clientY
  canvas.value!.style.cursor = 'grabbing'

  // 注册移动 & 松开事件
  window.addEventListener('mousemove', handleMouseMove)
  window.addEventListener('mouseup', handleMouseUp)
}

// 鼠标移动：旋转视角
const handleMouseMove = (e: MouseEvent) => {
  if (!isDragging) return
  const deltaX = e.clientX - lastX
  const deltaY = e.clientY - lastY
  lastX = e.clientX
  lastY = e.clientY

  update_camera(deltaY * props.sensitivity, deltaX * props.sensitivity)  // 调用 Rust 更新摄像机角度
}

// 鼠标松开：停止拖拽
const handleMouseUp = () => {
  isDragging = false
  canvas.value!.style.cursor = 'grab'
  window.removeEventListener('mousemove', handleMouseMove)
  window.removeEventListener('mouseup', handleMouseUp)
}

// 滚轮：缩放
const handleWheel = (e: WheelEvent) => {
  zoom_camera(e.deltaY * 0.01) // 调节缩放速率
  e.preventDefault()
}

// 生命周期
onMounted(() => {
  if (canvas.value) {
    canvas.value.width = props.width
    canvas.value.height = props.height
    canvas.value.style.cursor = 'grab'
    setupWasm()  // 初始化 WASM 和纹理
  }
})

onUnmounted(() => {
  window.removeEventListener('mousemove', handleMouseMove)
  window.removeEventListener('mouseup', handleMouseUp)
})
</script>

<style scoped>
.skybox {
  display: block;
  background: #000;
  user-select: none;
}
</style>
