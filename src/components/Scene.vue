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
import init, {
  initialize,
  update_camera,
  zoom_camera,
  render,
  set_viewport_size,
} from '../pkg/black_hole_wasm.js'

const props = defineProps({
  width: { type: Number, default: 800 },
  height: { type: Number, default: 600 },
  sensitivity: { type: Number, default: 0.15 },
})

const canvas = ref<HTMLCanvasElement | null>(null)
let isDragging = false
let lastX = 0
let lastY = 0

const setupWasm = async () => {
  await init()
  initialize()
  updateViewport()
  startRenderLoop()
}

const startRenderLoop = () => {
  const ctx = canvas.value!.getContext('2d')!
  const animate = () => {
    render(ctx)
    requestAnimationFrame(animate)
  }
  requestAnimationFrame(animate)
}

const updateViewport = () => {
  if (!canvas.value) return

  const ratio = 0.8
  const width = window.innerWidth * ratio
  const height = window.innerHeight * ratio
  console.log(`ts: width: ${width} height: ${height}`);
  canvas.value.width = width
  canvas.value.height = height

  set_viewport_size(width, height)
}

const handleMouseDown = (e: MouseEvent) => {
  isDragging = true
  lastX = e.clientX
  lastY = e.clientY
  canvas.value!.style.cursor = 'grabbing'

  window.addEventListener('mousemove', handleMouseMove)
  window.addEventListener('mouseup', handleMouseUp)
}

const handleMouseMove = (e: MouseEvent) => {
  if (!isDragging) return
  const deltaX = e.clientX - lastX
  const deltaY = lastY - e.clientY
  lastX = e.clientX
  lastY = e.clientY

  update_camera(deltaY * props.sensitivity, deltaX * props.sensitivity)
}

const handleMouseUp = () => {
  isDragging = false
  canvas.value!.style.cursor = 'grab'
  window.removeEventListener('mousemove', handleMouseMove)
  window.removeEventListener('mouseup', handleMouseUp)
}

const handleWheel = (e: WheelEvent) => {
  zoom_camera(e.deltaY * 0.01)
  e.preventDefault()
}

window.addEventListener('resize', updateViewport)

onMounted(() => {
  if (canvas.value) {
    canvas.value.width = props.width
    canvas.value.height = props.height
    canvas.value.style.cursor = 'grab'
    setupWasm()
  }
})

onUnmounted(() => {
  window.removeEventListener('mousemove', handleMouseMove)
  window.removeEventListener('mouseup', handleMouseUp)
  window.removeEventListener('resize', updateViewport)
})
</script>

<style scoped>
.skybox {
  display: block;
  background: #000;
  user-select: none;
}
</style>
