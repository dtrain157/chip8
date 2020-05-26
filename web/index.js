import * as wasm from "chip8";
import { memory } from "chip8/chip8_bg";

/**** HELPER FUCTIONS ****/
const hex = (value, length = 2) => {
  const padded = "0000" + value.toString(16).toUpperCase();
  return padded.substr(padded.length - length);
};

/**** TURN ON THE EMULATOR ****/
const chip8 = wasm.Chip8.power_up();

/**** SET UP DISPLAY ****/
const DISPLAY_WIDTH = chip8.get_display_width();
const DISPLAY_HEIHT = chip8.get_display_height();

const displayPtr = chip8.get_display_memory();
const displayMemory = new Uint8Array(memory.buffer, displayPtr, DISPLAY_WIDTH * DISPLAY_HEIHT);

const canvas = document.getElementById("chip8-display-canvas");
const ctx = canvas.getContext("2d");

const updateDisplay = () => {
  const imageData = ctx.createImageData(DISPLAY_WIDTH, DISPLAY_HEIHT);
  for (let i = 0; i < displayMemory.length; i++) {
    imageData.data[i * 4] = displayMemory[i] === 1 ? 0xff : 0;
    imageData.data[i * 4 + 1] = displayMemory[i] === 1 ? 0xff : 0;
    imageData.data[i * 4 + 2] = displayMemory[i] === 1 ? 0xff : 0;
    imageData.data[i * 4 + 3] = 0xff;
  }
  ctx.putImageData(imageData, 0, 0);
};

/**** SET UP MAIN MEMORY ****/
const MEMORY_SIZE = chip8.get_memory_size();

const memoryPtr = chip8.get_memory();
const mainMemory = new Uint8Array(memory.buffer, memoryPtr, MEMORY_SIZE);

const get_opcode_from_memory = (pc) => {
  return (mainMemory[pc] << 8) | mainMemory[pc + 1];
};

/**** SET UP V-REGISTERS ****/
const vRegisterPtr = chip8.get_v_registers();
const vRegisters = new Uint8Array(memory.buffer, vRegisterPtr, 16);

/**** SET UP OPCODE DISASSEMBLER ****/
const inRange = (value, lower, upper) => value >= lower && value <= upper;

const dissassemble_opcode = (opcode) => {
  const x = (opcode & 0x0f00) >> 8;
  const y = (opcode & 0x00f0) >> 4;
  const nnn = opcode & 0x0fff;
  const kk = opcode & 0x00ff;
  const n = opcode & 0x000f;

  if (opcode === 0x00e0) return "CLS";
  if (opcode === 0x00ee) return "RET";
  if (inRange(opcode, 0x1000, 0x1fff)) return `JP 0x${hex(nnn, 3)}`;
  if (inRange(opcode, 0x2000, 0x2fff)) return `CALL 0x${hex(nnn, 3)}`;
  if (inRange(opcode, 0x3000, 0x3fff)) return `SE V${n} ${hex(kk)}`;
  if (inRange(opcode, 0x4000, 0x4fff)) return `SNE V${n} ${hex(kk)}`;
  if (inRange(opcode, 0x5000, 0x5fff)) return `SE V${x} V${y}`;
  if (inRange(opcode, 0x6000, 0x6fff)) return `LD V${x} ${hex(kk)}`;
  if (inRange(opcode, 0x7000, 0x7fff)) return `ADD V${x} ${hex(kk)}`;
  if (inRange(opcode, 0x8000, 0x8fff)) {
    if (n === 0x0) return `LD V${x} V${y}`;
    if (n === 0x1) return `OR V${x} V${y}`;
    if (n === 0x2) return `AND V${x} V${y}`;
    if (n === 0x3) return `XOR V${x} V${y}`;
    if (n === 0x4) return `ADD V${x} V${y}`;
    if (n === 0x5) return `SUB V${x} V${y}`;
    if (n === 0x6) return `SHR V${x}`;
    if (n === 0x7) return `SUBN V${x} V${y}`;
    if (n === 0xe) return `SHL V${x}`;
  }
  if (inRange(opcode, 0x9000, 0x9fff)) return `SNE V${x} V${y}`;
  if (inRange(opcode, 0xa000, 0xafff)) return `LDI ${hex(nnn, 3)}`;
  if (inRange(opcode, 0xb000, 0xbfff)) return `JP V0 + ${hex(nnn, 3)}`;
  if (inRange(opcode, 0xc000, 0xcfff)) return `RND ${hex(kk)}`;
  if (inRange(opcode, 0xd000, 0xdfff)) return `DRW V${x} V${y} ${n}`;
  if (inRange(opcode, 0xe000, 0xefff)) {
    if (kk === 0x9e) return `SKP V${x}`;
    if (kk === 0xa1) return `SKNP V${x}`;
  }
  if (inRange(opcode, 0xf000, 0xffff)) {
    if (kk === 0x07) return `LD V${x} DT`;
    if (kk === 0x0a) return `LD V${x} K`;
    if (kk === 0x15) return `LD DT, V${x}`;
    if (kk === 0x1e) return `ADD I, V${x}`;
    if (kk === 0x29) return `LD F, V${x}`;
    if (kk === 0x33) return `LD B, V${x}`;
    if (kk === 0x55) return `LD [I], ${x}`;
    if (kk === 0x65) return `LD ${x}, [I]`;
  }
  return "-";
};

/**** OUTPUT FUNCTIONS ****/
const write_current_opcode = () => {
  const pc = chip8.get_pc();
  const opcode = get_opcode_from_memory(pc);
  document.getElementById("opcode").innerHTML = `Current Opcode: ${hex(opcode, 4)} [ ${dissassemble_opcode(opcode)} ]`;
};

const write_registers = () => {
  document.getElementById("v_registers").innerHTML = "";
  for (var i = 0; i < 16; i++) {
    document.getElementById("v_registers").innerHTML += `V${hex(i, 1)}: ${hex(vRegisters[i])}<br/>`;
  }

  document.getElementById("pc").innerHTML = `PC: ${hex(chip8.get_pc(), 4)}`;
  document.getElementById("i").innerHTML = ` I: ${hex(chip8.get_i(), 4)}`;
  document.getElementById("delay_timer").innerHTML = `DT: ${hex(chip8.get_delay_timer())}`;
  document.getElementById("sound_timer").innerHTML = `ST: ${hex(chip8.get_sound_timer())}`;
};

/**** BUTTONS ****/
var is_step_through = false;
document.getElementById("step").onclick = function () {
  is_step_through = document.getElementById("step").checked;
  if (is_step_through) {
    document.getElementById("go_button").value = "Step";
  } else {
    document.getElementById("go_button").value = "Run";
  }
};

document.getElementById("go_button").onclick = function () {
  renderLoop();
};

/**** EMULATION LOOP ****/
function renderLoop() {
  if (is_step_through) {
    //if we're stepping through, only execute one cycle every frame
    chip8.execute_cycle();
  } else {
    //otherwise, execute 8 cycles on every frame (We want to run the emulation at close to 500Hz,
    // which is the normal operating clockspeed of the chip8. Since requestAnimationFrame() runs at
    // 60fps, executing 8 cycles per frame will give us a clockspeed of 480Hz).
    for (var i = 0; i < 8; i++) {
      chip8.execute_cycle();
    }
  }

  chip8.decrement_timers();

  write_current_opcode();
  write_registers();
  updateDisplay();

  if (!is_step_through) {
    requestAnimationFrame(renderLoop);
  }
}

write_current_opcode();
write_registers();
updateDisplay();
