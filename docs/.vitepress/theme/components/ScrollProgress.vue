<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'

const progress = ref(0)
const hovering = ref(false)

const percentage = computed(() => Math.round(progress.value * 100))
const arcDegree = computed(() => `${(progress.value * 360).toFixed(1)}deg`)
const isGoTop = computed(() => percentage.value > 20)
const ariaLabel = computed(() => (isGoTop.value ? '回到页面顶部' : '跳到页面底部'))
const iconPath = computed(() => (isGoTop.value ? 'M12 19V5M5 12l7-7 7 7M7 19h10' : 'M12 5v14M5 12l7 7 7-7M7 5h10'))

const ringStyle = computed(() => ({
  '--scroll-progress-degree': arcDegree.value,
}))

function updateProgress() {
  const { scrollTop, scrollHeight, clientHeight } = document.documentElement
  const total = scrollHeight - clientHeight
  const ratio = total > 0 ? scrollTop / total : 0
  progress.value = Math.min(1, Math.max(0, ratio))
}

function handleClick() {
  const targetTop = isGoTop.value ? 0 : document.documentElement.scrollHeight - window.innerHeight
  window.scrollTo({ top: targetTop, behavior: 'smooth' })
}

onMounted(() => {
  updateProgress()
  window.addEventListener('scroll', updateProgress, { passive: true })
  window.addEventListener('resize', updateProgress)
})

onUnmounted(() => {
  window.removeEventListener('scroll', updateProgress)
  window.removeEventListener('resize', updateProgress)
})
</script>

<template>
  <button
    class="scroll-progress"
    type="button"
    :style="ringStyle"
    aria-live="polite"
    :aria-label="ariaLabel"
    @mouseenter="hovering = true"
    @mouseleave="hovering = false"
    @click="handleClick"
  >
    <span class="scroll-progress__percent" :class="{ 'is-hidden': hovering }">{{ percentage }}%</span>
    <span class="scroll-progress__arrow" :class="{ 'is-visible': hovering }">
      <svg class="scroll-progress__icon" viewBox="0 0 24 24" role="presentation" aria-hidden="true">
        <path
          :d="iconPath"
          fill="none"
          stroke="currentColor"
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="1.8"
        />
      </svg>
    </span>
  </button>
</template>

<style scoped>
.scroll-progress {
  --scroll-progress-active: var(--vp-c-brand-1);
  --scroll-progress-track: color-mix(in srgb, var(--vp-c-text-1) 10%, transparent);
  --scroll-progress-surface: color-mix(in srgb, var(--vp-c-bg) 92%, white);
  --scroll-progress-foreground: var(--vp-c-text-1);
  --scroll-progress-shadow: 0 12px 26px color-mix(in srgb, var(--vp-c-text-1) 22%, transparent);
  position: fixed;
  right: 2rem;
  bottom: 2rem;
  width: 56px;
  height: 56px;
  border: none;
  border-radius: 50%;
  background: conic-gradient(
      var(--scroll-progress-active) 0deg,
      var(--scroll-progress-active) var(--scroll-progress-degree, 0deg),
      var(--scroll-progress-track) var(--scroll-progress-degree, 0deg),
      var(--scroll-progress-track) 360deg
    )
    center center / cover no-repeat;
  color: var(--scroll-progress-foreground);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: var(--scroll-progress-shadow);
  transition:
    transform 0.2s ease,
    box-shadow 0.2s ease;
  z-index: 30;
  padding: 0;
}

.dark .scroll-progress {
  --scroll-progress-track: color-mix(in srgb, white 12%, transparent);
  --scroll-progress-surface: color-mix(in srgb, var(--vp-c-bg) 90%, black);
  --scroll-progress-shadow: 0 14px 30px color-mix(in srgb, black 45%, transparent);
}

.scroll-progress::after {
  content: '';
  position: absolute;
  inset: 6px;
  border-radius: 50%;
  background: var(--scroll-progress-surface);
  box-shadow: inset 0 1px 2px color-mix(in srgb, white 14%, transparent);
}

.scroll-progress:focus-visible {
  outline: 2px solid var(--scroll-progress-active);
  outline-offset: 2px;
}

.scroll-progress:hover {
  transform: translateY(-2px) scale(1.02);
  box-shadow: 0 14px 32px color-mix(in srgb, black 40%, transparent);
}

.scroll-progress__percent,
.scroll-progress__arrow {
  position: absolute;
  inset: 0;
  z-index: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.95rem;
  font-weight: 600;
  line-height: 1;
  letter-spacing: 0.4px;
  transition:
    opacity 0.18s ease,
    transform 0.18s ease;
}

.scroll-progress__percent.is-hidden {
  opacity: 0;
  transform: translateY(4px);
}

.scroll-progress__arrow {
  opacity: 0;
  transform: scale(0.85);
}

.scroll-progress__arrow.is-visible {
  opacity: 1;
  transform: scale(1);
}

.scroll-progress__icon {
  width: 24px;
  height: 24px;
}

@media (max-width: 768px) {
  .scroll-progress {
    right: 1.25rem;
    bottom: 1.25rem;
    width: 50px;
    height: 50px;
  }

  .scroll-progress::after {
    inset: 5px;
  }

  .scroll-progress__icon {
    width: 22px;
    height: 22px;
  }
}
</style>
