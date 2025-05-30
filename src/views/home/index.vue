<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { use } from 'echarts/core'
import { CanvasRenderer } from 'echarts/renderers'
import { LineChart, BarChart, PieChart, RadarChart, GaugeChart, FunnelChart, HeatmapChart } from 'echarts/charts'
import {
  TitleComponent,
  TooltipComponent,
  LegendComponent,
  GridComponent,
  RadarComponent,
  VisualMapComponent
} from 'echarts/components'
import BaseChart from '@/components/charts/BaseChart.vue'
import { theme } from '@/theme/theme'

const { t } = useI18n()

use([
  CanvasRenderer,
  LineChart,
  BarChart,
  PieChart,
  RadarChart,
  TitleComponent,
  TooltipComponent,
  LegendComponent,
  GridComponent,
  RadarComponent,
  GaugeChart,
  FunnelChart,
  HeatmapChart,
  VisualMapComponent
])

const stats = computed(() => [
  { icon: '👥', label: t('home.newVisits'), value: '102,400', color: 'text-cyan-500', colorValue: '#22d3ee' },
  { icon: '💬', label: t('home.messages'), value: '81,212', color: 'text-blue-400', colorValue: '#60a5fa' },
  { icon: '￥', label: t('home.purchases'), value: '9,280', color: 'text-pink-400', colorValue: '#f472b6' },
  { icon: '🛒', label: t('home.shoppings'), value: '13,600', color: 'text-green-500', colorValue: '#4ade80' },
])

const lineChartOption = computed(() => ({
  tooltip: { trigger: 'axis' },
  legend: { data: [t('home.expected'), t('home.actual')], top: 10 },
  grid: { left: 40, right: 20, bottom: 40, top: 40 },
  xAxis: {
    type: 'category',
    boundaryGap: false,
    data: [t('home.mon'), t('home.tue'), t('home.wed'), t('home.thu'), t('home.fri'), t('home.sat'), t('home.sun')]
  },
  yAxis: { type: 'value' },
  series: [
    {
      name: t('home.expected'),
      type: 'line',
      smooth: true,
      data: [120, 132, 161, 134, 105, 160, 165],
      lineStyle: { color: '#f2637b' },
      itemStyle: { color: '#f2637b' }
    },
    {
      name: t('home.actual'),
      type: 'line',
      smooth: true,
      data: [100, 90, 91, 104, 130, 150, 160],
      lineStyle: { color: '#4a90e2' },
      itemStyle: { color: '#4a90e2' }
    }
  ]
}))

const radarChartOption = computed(() => ({
  tooltip: {},
  legend: { data: [t('home.allocatedBudget'), t('home.expectedSpending'), t('home.actualSpending')] },
  radar: {
    indicator: [
      { name: t('home.sales'), max: 6500 },
      { name: t('home.administration'), max: 16000 },
      { name: t('home.it'), max: 30000 },
      { name: t('home.customerSupport'), max: 38000 },
      { name: t('home.development'), max: 52000 },
      { name: t('home.marketing'), max: 25000 }
    ]
  },
  series: [{
    type: 'radar',
    data: [
      { value: [4200, 3000, 20000, 35000, 50000, 18000], name: t('home.allocatedBudget') },
      { value: [5000, 14000, 28000, 26000, 42000, 21000], name: t('home.expectedSpending') },
      { value: [3000, 12000, 25000, 30000, 40000, 15000], name: t('home.actualSpending') }
    ]
  }]
}))

const pieChartOption = computed(() => ({
  tooltip: { trigger: 'item' },
  legend: { orient: 'vertical', left: 'left' },
  series: [{
    name: t('home.accessFrom'),
    type: 'pie',
    radius: ['40%', '70%'],
    avoidLabelOverlap: false,
    itemStyle: { borderRadius: 10, borderColor: '#fff', borderWidth: 2 },
    label: { show: false, position: 'center' },
    emphasis: { label: { show: true, fontSize: 18, fontWeight: 'bold' } },
    labelLine: { show: false },
    data: [
      { value: 1048, name: t('home.industries') },
      { value: 735, name: t('home.technology') },
      { value: 580, name: t('home.forex') },
      { value: 484, name: t('home.gold') },
      { value: 300, name: t('home.forecasts') }
    ]
  }]
}))

const barChartOption = computed(() => ({
  tooltip: { trigger: 'axis' },
  legend: { data: [t('home.industries'), t('home.technology'), t('home.forex')] },
  grid: { left: 40, right: 20, bottom: 40, top: 40 },
  xAxis: { type: 'category', data: [t('home.mon'), t('home.tue'), t('home.wed'), t('home.thu'), t('home.fri'), t('home.sat'), t('home.sun')] },
  yAxis: { type: 'value' },
  series: [
    {
      name: t('home.industries'),
      type: 'bar',
      stack: 'total',
      emphasis: { focus: 'series' },
      data: [320, 332, 301, 334, 390, 330, 320],
      itemStyle: { color: '#4ad991' }
    },
    {
      name: t('home.technology'),
      type: 'bar',
      stack: 'total',
      emphasis: { focus: 'series' },
      data: [120, 132, 101, 134, 90, 230, 210],
      itemStyle: { color: '#a389f4' }
    },
    {
      name: t('home.forex'),
      type: 'bar',
      stack: 'total',
      emphasis: { focus: 'series' },
      data: [220, 182, 191, 234, 290, 330, 310],
      itemStyle: { color: '#4a90e2' }
    }
  ]
}))

const gaugeOption = computed(() => ({
  tooltip: { formatter: '{a} <br/>{b} : {c}%' },
  series: [
    {
      name: t('home.completion'),
      type: 'gauge',
      progress: { show: true },
      detail: { valueAnimation: true, formatter: '{value}%' },
      data: [{ value: 78, name: t('home.progress') }],
      axisLine: { lineStyle: { width: 12, color: [[0.3, '#4ad991'], [0.7, '#4a90e2'], [1, '#f2637b']] } }
    }
  ]
}))

const funnelOption = computed(() => ({
  tooltip: { trigger: 'item', formatter: '{b} : {c}%' },
  series: [
    {
      name: t('home.funnel'),
      type: 'funnel',
      left: '10%',
      top: 20,
      bottom: 20,
      width: '80%',
      min: 0,
      max: 100,
      minSize: '0%',
      maxSize: '100%',
      sort: 'descending',
      gap: 2,
      label: { show: true, position: 'inside' },
      labelLine: { length: 10, lineStyle: { width: 1, type: 'solid' } },
      itemStyle: { borderColor: '#fff', borderWidth: 1 },
      emphasis: { label: { fontSize: 16 } },
      data: [
        { value: 90, name: t('home.show') },
        { value: 70, name: t('home.click') },
        { value: 50, name: t('home.visit') },
        { value: 30, name: t('home.inquiry') },
        { value: 10, name: t('home.order') }
      ]
    }
  ]
}))

const heatmapOption = computed(() => ({
  tooltip: { position: 'top' },
  grid: { height: '60%', top: '10%' },
  xAxis: { type: 'category', data: [t('home.mon'), t('home.tue'), t('home.wed'), t('home.thu'), t('home.fri'), t('home.sat'), t('home.sun')], splitArea: { show: true } },
  yAxis: { type: 'category', data: ['1', '2', '3', '4', '5'], splitArea: { show: true } },
  visualMap: { min: 0, max: 10, calculable: true, orient: 'horizontal', left: 'center', bottom: '5%' },
  series: [{
    name: t('home.punchCard'),
    type: 'heatmap',
    data: Array.from({ length: 35 }, (_, i) => [i % 7, Math.floor(i / 7), Math.floor(Math.random() * 10)]),
    label: { show: true },
    emphasis: { itemStyle: { shadowBlur: 10, shadowColor: 'rgba(0,0,0,0.5)' } }
  }]
}))

const areaOption = computed(() => ({
  tooltip: { trigger: 'axis' },
  legend: { data: [t('home.email'), t('home.unionAds'), t('home.videoAds')] },
  grid: { left: '3%', right: '4%', bottom: '3%', containLabel: true },
  xAxis: [
    { type: 'category', boundaryGap: false, data: [t('home.mon'), t('home.tue'), t('home.wed'), t('home.thu'), t('home.fri'), t('home.sat'), t('home.sun')] }
  ],
  yAxis: [{ type: 'value' }],
  series: [
    {
      name: t('home.email'),
      type: 'line',
      stack: 'Total',
      areaStyle: {},
      emphasis: { focus: 'series' },
      data: [120, 132, 101, 134, 90, 230, 210],
      itemStyle: { color: '#4a90e2' }
    },
    {
      name: t('home.unionAds'),
      type: 'line',
      stack: 'Total',
      areaStyle: {},
      emphasis: { focus: 'series' },
      data: [220, 182, 191, 234, 290, 330, 310],
      itemStyle: { color: '#4ad991' }
    },
    {
      name: t('home.videoAds'),
      type: 'line',
      stack: 'Total',
      areaStyle: {},
      emphasis: { focus: 'series' },
      data: [150, 232, 201, 154, 190, 330, 410],
      itemStyle: { color: '#f2637b' }
    }
  ]
}))
</script>

<template>
  <div
    class="dashboard-bg min-h-screen p-4 md:p-8"
    :style="{
      background: theme.name === 'dark'
        ? 'linear-gradient(120deg, #181c2f 0%, #23263a 100%)'
        : '#f6f8fb',
      color: theme.name === 'dark' ? '#e5e6eb' : '#222',
      position: 'relative',
      overflow: 'hidden'
    }"
  >
    <!-- 更酷的SVG科技感背景 -->
    <svg v-if="theme.name === 'dark'" style="position:absolute;z-index:0;left:0;top:0;width:100%;height:100%;pointer-events:none;" width="100%" height="100%">
      <defs>
        <radialGradient id="bg-glow1" cx="80%" cy="20%" r="60%">
          <stop offset="0%" stop-color="#6b7cff" stop-opacity="0.28"/>
          <stop offset="100%" stop-color="#23263a" stop-opacity="0"/>
        </radialGradient>
        <radialGradient id="bg-glow2" cx="20%" cy="80%" r="50%">
          <stop offset="0%" stop-color="#00ffe7" stop-opacity="0.18"/>
          <stop offset="100%" stop-color="#23263a" stop-opacity="0"/>
        </radialGradient>
      </defs>
      <rect width="100%" height="100%" fill="url(#bg-glow1)" />
      <rect width="100%" height="100%" fill="url(#bg-glow2)" />
      <!-- 多条斜线 -->
      <line x1="0" y1="0" x2="100%" y2="100%" stroke="#6b7cff" stroke-opacity="0.08" stroke-width="2" />
      <line x1="100%" y1="0" x2="0" y2="100%" stroke="#00ffe7" stroke-opacity="0.06" stroke-width="2" />
      <!-- 圆环 -->
      <circle cx="90%" cy="10%" r="80" fill="none" stroke="#6b7cff" stroke-opacity="0.10" stroke-width="2"/>
      <circle cx="10%" cy="90%" r="60" fill="none" stroke="#00ffe7" stroke-opacity="0.08" stroke-width="2"/>
    </svg>
    <div style="position:relative;z-index:1;">
      <!-- 统计卡片 -->
      <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-8 mb-10">
        <div
          v-for="item in stats"
          :key="item.label"
          class="flex flex-col items-center justify-center p-6 min-w-0 min-h-[100px]"
          :style="{
            background: theme.name === 'dark' ? 'linear-gradient(135deg, #23263a 80%, #2d3146 100%)' : '#fff',
            color: theme.name === 'dark' ? '#f5f6fa' : '#222',
            borderRadius: '1.25rem',
            boxShadow: theme.name === 'dark'
              ? '0 6px 32px 0 rgba(30,40,90,0.55), 0 1.5px 0 0 #3a3f5c inset'
              : '0 2px 12px 0 rgba(0,0,0,0.06)',
            border: theme.name === 'dark'
              ? '1.5px solid rgba(120,140,255,0.10)'
              : '1px solid #f0f0f0',
            transition: 'background 0.4s, color 0.4s, box-shadow 0.4s, border 0.4s',
            overflow: 'hidden',
            position: 'relative'
          }"
        >
          <span
            :style="{ color: theme.name === 'dark' ? '#fff' : item.colorValue, fontSize: '2.2rem', marginBottom: '0.5rem' }"
          >{{ item.icon }}</span>
          <div
            :style="{ color: theme.name === 'dark' ? '#b0b3b8' : '#666', fontWeight: 500 }"
            class="text-xs md:text-sm mb-1 text-center break-words w-full"
          >{{ item.label }}</div>
          <div
            :style="{ color: theme.name === 'dark' ? '#fff' : '#222', fontWeight: 700, fontSize: theme.name === 'dark' ? '2.1rem' : '2rem' }"
            class="text-xl md:text-2xl font-bold text-center break-all w-full"
          >{{ item.value }}</div>
          <!-- 炫彩边框特效 -->
          <div v-if="theme.name === 'dark'" style="pointer-events:none;position:absolute;inset:0;border-radius:1.25rem;box-shadow:0 0 0 2px rgba(120,140,255,0.10),0 0 16px 0 rgba(120,140,255,0.08) inset;"></div>
        </div>
      </div>
      <!-- 主折线图 -->
      <div
        :style="{
          background: theme.name === 'dark'
            ? 'linear-gradient(135deg, #23263a 80%, #2d3146 100%)'
            : '#fff',
          color: theme.name === 'dark' ? '#f5f6fa' : '#222',
          borderRadius: '1.25rem',
          boxShadow: theme.name === 'dark'
            ? '0 6px 32px 0 rgba(30,40,90,0.55), 0 1.5px 0 0 #3a3f5c inset'
            : '0 2px 12px 0 rgba(0,0,0,0.06)',
          border: theme.name === 'dark'
            ? '1.5px solid rgba(120,140,255,0.10)'
            : '1px solid #f0f0f0',
          transition: 'background 0.4s, color 0.4s, box-shadow 0.4s, border 0.4s',
          overflow: 'hidden',
          position: 'relative',
          padding: '2.5rem 2rem 2rem 2rem',
          marginBottom: '2.5rem'
        }"
      >
        <BaseChart :option="lineChartOption" :naiveTheme="theme" :title="t('home.trend')" height="18rem" />
        <div v-if="theme.name === 'dark'" style="pointer-events:none;position:absolute;inset:0;border-radius:1.25rem;box-shadow:0 0 0 2px rgba(120,140,255,0.10),0 0 16px 0 rgba(120,140,255,0.08) inset;"></div>
      </div>
      <!-- 下方三图 -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
        <div
          :style="{
            background: theme.name === 'dark'
              ? 'linear-gradient(135deg, #23263a 80%, #2d3146 100%)'
              : '#fff',
            color: theme.name === 'dark' ? '#f5f6fa' : '#222',
            borderRadius: '1.25rem',
            boxShadow: theme.name === 'dark'
              ? '0 6px 32px 0 rgba(30,40,90,0.55), 0 1.5px 0 0 #3a3f5c inset'
              : '0 2px 12px 0 rgba(0,0,0,0.06)',
            border: theme.name === 'dark'
              ? '1.5px solid rgba(120,140,255,0.10)'
              : '1px solid #f0f0f0',
            transition: 'background 0.4s, color 0.4s, box-shadow 0.4s, border 0.4s',
            overflow: 'hidden',
            position: 'relative',
            padding: '2.5rem 2rem 2rem 2rem'
          }"
        >
          <BaseChart :option="radarChartOption" :naiveTheme="theme" :title="t('home.salesRadar')" height="15rem" />
          <div v-if="theme.name === 'dark'" style="pointer-events:none;position:absolute;inset:0;border-radius:1.25rem;box-shadow:0 0 0 2px rgba(120,140,255,0.10),0 0 16px 0 rgba(120,140,255,0.08) inset;"></div>
        </div>
        <div
          :style="{
            background: theme.name === 'dark'
              ? 'linear-gradient(135deg, #23263a 80%, #2d3146 100%)'
              : '#fff',
            color: theme.name === 'dark' ? '#f5f6fa' : '#222',
            borderRadius: '1.25rem',
            boxShadow: theme.name === 'dark'
              ? '0 6px 32px 0 rgba(30,40,90,0.55), 0 1.5px 0 0 #3a3f5c inset'
              : '0 2px 12px 0 rgba(0,0,0,0.06)',
            border: theme.name === 'dark'
              ? '1.5px solid rgba(120,140,255,0.10)'
              : '1px solid #f0f0f0',
            transition: 'background 0.4s, color 0.4s, box-shadow 0.4s, border 0.4s',
            overflow: 'hidden',
            position: 'relative',
            padding: '2.5rem 2rem 2rem 2rem'
          }"
        >
          <BaseChart :option="pieChartOption" :naiveTheme="theme" :title="t('home.marketShare')" height="15rem" />
          <div v-if="theme.name === 'dark'" style="pointer-events:none;position:absolute;inset:0;border-radius:1.25rem;box-shadow:0 0 0 2px rgba(120,140,255,0.10),0 0 16px 0 rgba(120,140,255,0.08) inset;"></div>
        </div>
        <div
          :style="{
            background: theme.name === 'dark'
              ? 'linear-gradient(135deg, #23263a 80%, #2d3146 100%)'
              : '#fff',
            color: theme.name === 'dark' ? '#f5f6fa' : '#222',
            borderRadius: '1.25rem',
            boxShadow: theme.name === 'dark'
              ? '0 6px 32px 0 rgba(30,40,90,0.55), 0 1.5px 0 0 #3a3f5c inset'
              : '0 2px 12px 0 rgba(0,0,0,0.06)',
            border: theme.name === 'dark'
              ? '1.5px solid rgba(120,140,255,0.10)'
              : '1px solid #f0f0f0',
            transition: 'background 0.4s, color 0.4s, box-shadow 0.4s, border 0.4s',
            overflow: 'hidden',
            position: 'relative',
            padding: '2.5rem 2rem 2rem 2rem'
          }"
        >
          <BaseChart :option="barChartOption" :naiveTheme="theme" :title="t('home.weeklyData')" height="15rem" />
          <div v-if="theme.name === 'dark'" style="pointer-events:none;position:absolute;inset:0;border-radius:1.25rem;box-shadow:0 0 0 2px rgba(120,140,255,0.10),0 0 16px 0 rgba(120,140,255,0.08) inset;"></div>
        </div>
      </div>
      <!-- 高级可视化图表区 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-8 mt-10">
        <div
          :style="{
            background: theme.name === 'dark'
              ? 'linear-gradient(135deg, #23263a 80%, #2d3146 100%)'
              : '#fff',
            color: theme.name === 'dark' ? '#f5f6fa' : '#222',
            borderRadius: '1.25rem',
            boxShadow: theme.name === 'dark'
              ? '0 6px 32px 0 rgba(30,40,90,0.55), 0 1.5px 0 0 #3a3f5c inset'
              : '0 2px 12px 0 rgba(0,0,0,0.06)',
            border: theme.name === 'dark'
              ? '1.5px solid rgba(120,140,255,0.10)'
              : '1px solid #f0f0f0',
            transition: 'background 0.4s, color 0.4s, box-shadow 0.4s, border 0.4s',
            overflow: 'hidden',
            position: 'relative',
            padding: '2.5rem 2rem 2rem 2rem'
          }"
        >
          <BaseChart :option="gaugeOption" :naiveTheme="theme" :title="t('home.gauge')" height="12rem" />
          <div v-if="theme.name === 'dark'" style="pointer-events:none;position:absolute;inset:0;border-radius:1.25rem;box-shadow:0 0 0 2px rgba(120,140,255,0.10),0 0 16px 0 rgba(120,140,255,0.08) inset;"></div>
        </div>
        <div
          :style="{
            background: theme.name === 'dark'
              ? 'linear-gradient(135deg, #23263a 80%, #2d3146 100%)'
              : '#fff',
            color: theme.name === 'dark' ? '#f5f6fa' : '#222',
            borderRadius: '1.25rem',
            boxShadow: theme.name === 'dark'
              ? '0 6px 32px 0 rgba(30,40,90,0.55), 0 1.5px 0 0 #3a3f5c inset'
              : '0 2px 12px 0 rgba(0,0,0,0.06)',
            border: theme.name === 'dark'
              ? '1.5px solid rgba(120,140,255,0.10)'
              : '1px solid #f0f0f0',
            transition: 'background 0.4s, color 0.4s, box-shadow 0.4s, border 0.4s',
            overflow: 'hidden',
            position: 'relative',
            padding: '2.5rem 2rem 2rem 2rem'
          }"
        >
          <BaseChart :option="funnelOption" :naiveTheme="theme" :title="t('home.funnel')" height="12rem" />
          <div v-if="theme.name === 'dark'" style="pointer-events:none;position:absolute;inset:0;border-radius:1.25rem;box-shadow:0 0 0 2px rgba(120,140,255,0.10),0 0 16px 0 rgba(120,140,255,0.08) inset;"></div>
        </div>
        <div
          :style="{
            background: theme.name === 'dark'
              ? 'linear-gradient(135deg, #23263a 80%, #2d3146 100%)'
              : '#fff',
            color: theme.name === 'dark' ? '#f5f6fa' : '#222',
            borderRadius: '1.25rem',
            boxShadow: theme.name === 'dark'
              ? '0 6px 32px 0 rgba(30,40,90,0.55), 0 1.5px 0 0 #3a3f5c inset'
              : '0 2px 12px 0 rgba(0,0,0,0.06)',
            border: theme.name === 'dark'
              ? '1.5px solid rgba(120,140,255,0.10)'
              : '1px solid #f0f0f0',
            transition: 'background 0.4s, color 0.4s, box-shadow 0.4s, border 0.4s',
            overflow: 'hidden',
            position: 'relative',
            padding: '2.5rem 2rem 2rem 2rem'
          }"
        >
          <BaseChart :option="heatmapOption" :naiveTheme="theme" :title="t('home.heatmap')" height="12rem" />
          <div v-if="theme.name === 'dark'" style="pointer-events:none;position:absolute;inset:0;border-radius:1.25rem;box-shadow:0 0 0 2px rgba(120,140,255,0.10),0 0 16px 0 rgba(120,140,255,0.08) inset;"></div>
        </div>
        <div
          :style="{
            background: theme.name === 'dark'
              ? 'linear-gradient(135deg, #23263a 80%, #2d3146 100%)'
              : '#fff',
            color: theme.name === 'dark' ? '#f5f6fa' : '#222',
            borderRadius: '1.25rem',
            boxShadow: theme.name === 'dark'
              ? '0 6px 32px 0 rgba(30,40,90,0.55), 0 1.5px 0 0 #3a3f5c inset'
              : '0 2px 12px 0 rgba(0,0,0,0.06)',
            border: theme.name === 'dark'
              ? '1.5px solid rgba(120,140,255,0.10)'
              : '1px solid #f0f0f0',
            transition: 'background 0.4s, color 0.4s, box-shadow 0.4s, border 0.4s',
            overflow: 'hidden',
            position: 'relative',
            padding: '2.5rem 2rem 2rem 2rem'
          }"
        >
          <BaseChart :option="areaOption" :naiveTheme="theme" :title="t('home.stackedArea')" height="12rem" />
          <div v-if="theme.name === 'dark'" style="pointer-events:none;position:absolute;inset:0;border-radius:1.25rem;box-shadow:0 0 0 2px rgba(120,140,255,0.10),0 0 16px 0 rgba(120,140,255,0.08) inset;"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="less">
.dashboard-bg {
  background: #f6f8fb;
}
:deep(.echarts) {
  width: 100%;
  height: 100%;
}
.bg-white {
  box-shadow: 0 2px 12px 0 rgba(0,0,0,0.06);
  border-radius: 1rem;
  transition: background 0.3s;
}
</style>