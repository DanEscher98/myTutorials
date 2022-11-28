import { Universe, Cell } from "wasmconway";
import { memory } from "wasmconway/wasmconway_bg.wasm";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// construct the Universe and get its width and height
const universe = Universe.new();
const univ_height = universe.height();
const univ_width = universe.width();

const canvas = document.getElementById("conwaygame-canvas");
canvas.height = (CELL_SIZE + 1) * univ_height + 1;
canvas.width = (CELL_SIZE + 1) * univ_width + 1;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
  universe.tick();

  //drawGrid();
  drawCells();

  requestAnimationFrame(renderLoop);
}

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokesStyle = GRID_COLOR;

  // vertical lines
  for (let i = 0; i <= univ_width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * univ_height + 1);
  }

  // horizontal lines
  for (let j = 0; j <= univ_height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * univ_width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const getIndex = (row, column) => {
  return row * univ_width + column;
}

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, univ_width*univ_height);

  ctx.beginPath();

  for (let row = 0; row < univ_height; row++) {
    for (let col = 0; col < univ_width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === Cell.Dead
        ? DEAD_COLOR
        : ALIVE_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) * 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

// drawGrid();
drawCells();
requestAnimationFrame(renderLoop);
