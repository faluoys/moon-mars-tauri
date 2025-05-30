<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted, nextTick } from 'vue'
import { debounce } from '@/utils'
import VChart from 'vue-echarts'

const props = defineProps<{
  option: any
  title?: string
  height?: string
  naiveTheme?: any
}>()

const chartTheme = computed(() => {
  if (props.naiveTheme && props.naiveTheme.name === 'dark') return 'dark'
  if (props.naiveTheme && props.naiveTheme.name === 'light') return 'light'
  // fallback: 检查 html class
  return document.documentElement.classList.contains('dark') ? 'dark' : 'light'
})

const chartRef = ref<any>(null)

const handleResize = debounce(() => {
  if (chartRef.value && chartRef.value.resize) {
    chartRef.value.resize()
  }
}, 200)

onMounted(() => {
  window.addEventListener('resize', handleResize)
  // 主题切换等场景下也触发一次
  nextTick(() => handleResize())
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
})
</script>

<template>
  <div
    class="base-chart-card"
    :style="{
      background: chartTheme === 'dark' ? 'linear-gradient(135deg, #23263a 80%, #2d3146 100%)' : '#fff',
      color: chartTheme === 'dark' ? '#f5f6fa' : '#222',
      borderRadius: '1.25rem',
      boxShadow: chartTheme === 'dark'
        ? '0 6px 32px 0 rgba(30,40,90,0.55), 0 1.5px 0 0 #3a3f5c inset'
        : '0 2px 12px 0 rgba(0,0,0,0.06)',
      border: chartTheme === 'dark'
        ? '1.5px solid rgba(120,140,255,0.10)'
        : '1px solid #f0f0f0',
      transition: 'background 0.4s, color 0.4s, box-shadow 0.4s, border 0.4s',
      overflow: 'hidden',
      position: 'relative'
    }"
  >
    <div v-if="title" class="chart-title" :style="{color: chartTheme === 'dark' ? '#b3bfff' : '#888'}">{{ title }}</div>
    <v-chart ref="chartRef" :option="option" :autoresize="true" :theme="chartTheme" :style="{height: height || '16rem', width: '100%'}" />
    <!-- 炫彩边框特效 -->
    <div v-if="chartTheme === 'dark'" style="pointer-events:none;position:absolute;inset:0;border-radius:1.25rem;box-shadow:0 0 0 2px rgba(120,140,255,0.10),0 0 16px 0 rgba(120,140,255,0.08) inset;"></div>
  </div>
</template>

<style scoped>
.base-chart-card {
  padding: 1.5rem 1rem 1rem 1rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  transition: background 0.4s, color 0.4s, box-shadow 0.4s, border 0.4s;
}
.chart-title {
  font-size: 1rem;
  margin-bottom: 0.5rem;
  text-align: center;
  font-weight: 600;
  letter-spacing: 0.02em;
}
</style> 