import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 10; // px
const CELL_BORDER = 2; // px
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

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + CELL_BORDER)), height - CELL_BORDER);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + CELL_BORDER)), width - CELL_BORDER);

  universe.toggle_cell(row, col);

  drawGrid();
  drawCells();
});

const renderLoop = () => {
  // debugger;
  universe.tick();

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

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = bitIsSet(idx, cells)
        ? ALIVE_COLOR
        : DEAD_COLOR;

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
