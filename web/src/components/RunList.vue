<template>
  <div class="runs">
    <div class="title-bar">
      <div class="title-content">
        <h1>⛷️ Ski Buddy - Carving Runs</h1>
        <a href="http://localhost:8080/api/" target="_blank" class="api-link">API Docs</a>
      </div>
    </div>
    <div class="runs-content">
      <div v-for="run in runs" :key="run.id" class="run">
        <div class="run-header">
          <h2 class="run-title">Run {{ run.id }}</h2>
          <div class="run-date">{{ formatDateTime(run.start_time) }}</div>
        </div>
        <div class="run-path" ref="runPaths">
          <svg class="path-svg" viewBox="0 0 100 100" preserveAspectRatio="xMidYMid meet">
            <!-- Grid lines -->
            <line x1="0" y1="0" x2="100" y2="0" stroke="#eee" stroke-width="0.5" />
            <line x1="0" y1="100" x2="100" y2="100" stroke="#eee" stroke-width="0.5" />
            <line x1="0" y1="0" x2="0" y2="100" stroke="#eee" stroke-width="0.5" />
            <line x1="100" y1="0" x2="100" y2="100" stroke="#eee" stroke-width="0.5" />
            <!-- Paths -->
            <path :d="generatePath(run.turns, 0)" stroke="#007bff" stroke-width="3" fill="none" stroke-linecap="round"
              stroke-linejoin="round" :class="['ski-path', `ski-path-${run.id}`]" />
          </svg>
        </div>
        <div class="turns-header" @click="toggleTurns(run.id)">
          <h3>Turns ({{ run.turns.length }})</h3>
          <span class="toggle-icon" :class="{ 'is-open': openTurns[run.id] }">▼</span>
        </div>
        <div class="turns-container" :class="{ 'is-open': openTurns[run.id] }">
          <div v-for="turn in run.turns" :key="turn.timestamp_start" class="turn">
            <strong>{{ turn.direction }}</strong>
            <div class="metrics">
              <div class="metric">
                <div>Parallelness</div>
                <div class="metric-value">{{ turn.parallelness.toFixed(2) }}</div>
              </div>
              <div class="metric">
                <div>Closeness</div>
                <div class="metric-value">{{ turn.closeness.toFixed(2) }}</div>
              </div>
              <div class="metric">
                <div>Smoothness</div>
                <div class="metric-value">{{ turn.smoothness }}</div>
              </div>
              <div class="metric">
                <div>Duration</div>
                <div class="metric-value">{{ formatDuration(turn.timestamp_start, turn.timestamp_end) }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

const runs = ref([])
const openTurns = ref({})
const runPaths = ref([])

const formatDateTime = (timestamp) => {
  return new Date(timestamp).toLocaleString()
}

const formatDuration = (start, end) => {
  const startTime = new Date(start)
  const endTime = new Date(end)
  const durationSeconds = (endTime - startTime) / 1000
  return `${durationSeconds.toFixed(2)}s`
}

const generatePath = (turns, offset = 0) => {
  if (!turns || !turns.length) {
    console.log('No turns to visualize')
    return ''
  }

  const width = 100
  const height = 100
  const padding = 10
  const points = []

  // Start at the top, but offset based on first turn direction
  const firstTurnIsLeft = turns[0].direction.toLowerCase() === 'left'
  let currentX = width / 2 + (firstTurnIsLeft ? -30 / 2 : 30 / 2)
  let currentY = padding + offset

  const segmentWidth = 25 // Width of each turn
  const amplitude = 30 // How wide the turns go left/right
  const verticalProgress = (height - 2 * padding) / (turns.length + 1)

  // Create the initial point
  points.push({
    x: currentX,
    y: currentY,
    controlX1: currentX,
    controlY1: currentY,
    controlX2: currentX,
    controlY2: currentY + verticalProgress * 0.3
  })

  turns.forEach((turn, index) => {
    const isLeft = turn.direction.toLowerCase() === 'left'
    const targetX = currentX + (isLeft ? amplitude : -amplitude)

    // Add the turn point
    points.push({
      x: targetX,
      y: currentY + verticalProgress * 0.7,
      controlX1: currentX,
      controlY1: currentY + verticalProgress * 0.2,
      controlX2: targetX,
      controlY2: currentY + verticalProgress * 0.4
    })

    // Update position for next turn
    currentX = targetX
    currentY += verticalProgress
  })

  // Create SVG path
  let path = `M ${points[0].x} ${points[0].y}`
  for (let i = 1; i < points.length; i++) {
    path += ` C ${points[i - 1].controlX2} ${points[i - 1].controlY2}, ${points[i].controlX1} ${points[i].controlY1}, ${points[i].x} ${points[i].y}`
  }

  return path
}

const toggleTurns = (runId) => {
  openTurns.value[runId] = !openTurns.value[runId]
}

const fetchRuns = async () => {
  try {
    const response = await fetch('http://localhost:8080/api/runs')
    const data = await response.json()
    console.log('Fetched runs:', data)
    runs.value = data.data.runs
  } catch (error) {
    console.error('Error fetching runs:', error)
  }
}

onMounted(() => {
  fetchRuns()
})
</script>

<style scoped>
.runs {
  max-width: 99%;
  margin: 0 auto;
  padding: 0;
}

.title-bar {
  position: sticky;
  top: 0;
  z-index: 100;
  background: linear-gradient(to right, #2c5282, #2b6cb0);
  color: white;
  padding: 0.8rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  margin-bottom: 15px;
  border-radius: 0 0 8px 8px;
}

.title-content {
  max-width: 99%;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 10px;
}

.api-link {
  color: white;
  text-decoration: none;
  padding: 0.5rem 1rem;
  border: 1px solid rgba(255, 255, 255, 0.3);
  border-radius: 4px;
  transition: all 0.2s ease;
  font-weight: 500;
}

.api-link:hover {
  background: rgba(255, 255, 255, 0.1);
  border-color: rgba(255, 255, 255, 0.5);
}

.runs-content {
  padding: 10px;
}

.run {
  background-color: white;
  border-radius: 8px;
  padding: 15px;
  margin-bottom: 15px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.run-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
}

.run-title {
  font-size: 1.5em;
  margin: 0;
  color: #333;
}

.run-date {
  color: #666;
  font-size: 0.9em;
}

.run-path {
  margin: 15px 0;
  padding: 10px;
  background-color: #f8f9fa;
  border-radius: 4px;
  transform: perspective(1000px) rotateX(60deg);
  max-width: none;
}

.path-svg {
  width: 100%;
  height: 300px;
  background-color: white;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.turns-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px;
  background-color: #f8f9fa;
  border-radius: 4px;
  cursor: pointer;
  margin-top: 20px;
  transition: background-color 0.2s;
}

.turns-header:hover {
  background-color: #e9ecef;
}

.turns-header h3 {
  margin: 0;
}

.toggle-icon {
  transition: transform 0.3s ease;
}

.toggle-icon.is-open {
  transform: rotate(180deg);
}

.turns-container {
  max-height: 0;
  overflow: hidden;
  transition: max-height 0.3s ease-out;
}

.turns-container.is-open {
  max-height: 1500px;
  transition: max-height 0.5s ease-in;
}

.turn {
  background-color: #f8f9fa;
  border-radius: 4px;
  padding: 8px;
  margin: 6px 0;
  display: grid;
  grid-template-columns: auto 1fr;
  gap: 10px;
  align-items: center;
}

.metrics {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
}

.metric {
  background-color: white;
  padding: 8px;
  border-radius: 4px;
  text-align: center;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.metric>div:first-child {
  font-size: 0.8em;
  color: #666;
}

.metric-value {
  font-size: 1.1em;
  font-weight: bold;
  color: #007bff;
}

.ski-path {
  fill: none;
  stroke: #007bff;
  stroke-width: 3;
  stroke-linecap: round;
  stroke-linejoin: round;
}
</style>