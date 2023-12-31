import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 5; // px
const CELL_BORDER = 1; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new_random();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + CELL_BORDER) * height + CELL_BORDER;
canvas.width = (CELL_SIZE + CELL_BORDER) * width + CELL_BORDER;

const ctx = canvas.getContext('2d');

// Play/pause logic
let animationId = null;

const isPaused = () => {
  return animationId === null;
};

const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

const resetButton = document.getElementById("reset");

resetButton.textContent = "🔄";

resetButton.addEventListener("click", event => {
  universe.reset();
});

// Add click events
canvas.addEventListener("click", event => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + CELL_BORDER)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + CELL_BORDER)), width - 1);

  // TODO: add `event.shiftKey` to insert a pulsar
  if (event.ctrlKey) {
    universe.add_glider(row, col);
  } else if (event.shiftKey) {
    universe.add_pulsar(row, col);
  } else {
    universe.toggle_cell(row, col);
  }

  drawGrid();
  drawCells();
});

const fps = new class {
  constructor() {
    this.fps = document.getElementById("fps");
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = 1 / delta * 1000;

    // Save only the latest 100 timings.
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    // Find the max, min, and mean of our 100 latest timings.
    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    let mean = sum / this.frames.length;

    // Render the statistics.
    this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
  }
};

const ticks_per_frame = document.querySelector("#ticks-per-frame");
ticks_per_frame.addEventListener("input", (event) => {
  ticks_per_frame.value = event.target.value;
});

const renderLoop = () => {
  fps.render();
  
  // debugger;
  for (let i = 0; i < ticks_per_frame.value; i++) {
    universe.tick();
  }

  drawGrid();
  drawCells();

  animationId = requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + CELL_BORDER) + CELL_BORDER, 0);
    ctx.lineTo(i * (CELL_SIZE + CELL_BORDER) + CELL_BORDER, (CELL_SIZE + CELL_BORDER) * height + CELL_BORDER);
  }

  // Horizontal lines
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + CELL_BORDER) + CELL_BORDER);
    ctx.lineTo((CELL_SIZE + CELL_BORDER) * width + CELL_BORDER, j * (CELL_SIZE + CELL_BORDER) + CELL_BORDER);
  }

  ctx.stroke();
};

const getIndex = (row, column) => {
  return row * width + column;
};

const bitIsSet = (n, arr) => {
  const byte = Math.floor(n / 8);
  const mask = 1 << (n % 8);
  return (arr[byte] & mask) === mask;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);

  ctx.beginPath();

  // Alive cells
  ctx.fillStyle = ALIVE_COLOR;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);
      if (!bitIsSet(idx, cells)) {
        continue;
      }

      ctx.fillRect(
        col * (CELL_SIZE + CELL_BORDER) + CELL_BORDER,
        row * (CELL_SIZE + CELL_BORDER) + CELL_BORDER,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  // Dead cells
  ctx.fillStyle = DEAD_COLOR;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);
      if (bitIsSet(idx, cells)) {
        continue;
      }

      ctx.fillRect(
        col * (CELL_SIZE + CELL_BORDER) + CELL_BORDER,
        row * (CELL_SIZE + CELL_BORDER) + CELL_BORDER,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

drawGrid();
drawCells();
play();
