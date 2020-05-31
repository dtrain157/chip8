import * as wasm from "chip8";
import $ from "jquery";
import { memory } from "chip8/chip8_bg";

/**** COLOUR CONSTANTS ****/
const backgroundColour = getComputedStyle(document.body).backgroundColor;
const highlightColour = getComputedStyle(document.body).color;

/**** ROMS ****/
const ROMS = [
  "15PUZZLE",
  "BLINKY",
  "BLITZ",
  "BRIX",
  "CONNECT4",
  "GUESS",
  "HIDDEN",
  "IBM",
  "INVADERS",
  "KALEID",
  "MAZE",
  "MERLIN",
  "MISSILE",
  "PONG",
  "PONG2",
  "PUZZLE",
  "SYZYGY",
  "TANK",
  "TETRIS",
  "TICTAC",
  "UFO",
  "VBRIX",
  "VERS",
  "WIPEOFF"
];

ROMS.forEach(rom => {
  $("#roms").append(`<option value='${rom}'>${rom}</option>`);
});

/**** HELPER FUCTIONS ****/
const hex = (value, length = 2) => {
  const padded = "0000" + value.toString(16).toUpperCase();
  return padded.substr(padded.length - length);
};

const inRange = (value, lower, upper) => value >= lower && value <= upper;

/**** EMULATOR ****/
class Emulator {
  constructor() {
    /**** POWER UP CHIP8 EMULATOR ****/
    this.chip8 = wasm.Chip8.power_up();

    /**** SET UP DISPLAY INTERFACE ****/
    this.displayWidth = this.chip8.get_display_width();
    this.displayHeight = this.chip8.get_display_height();
    this.displayMemory = new Uint8Array(memory.buffer, this.chip8.get_display_memory(), this.displayWidth * this.displayHeight);
    this.canvas = document.getElementById("chip8-display");
    this.ctx = this.canvas.getContext("2d");

    /**** SET UP MAIN MEMORY INTERFACE ****/
    this.memorySize = this.chip8.get_memory_size();
    this.mainMemory = new Uint8Array(memory.buffer, this.chip8.get_memory(), this.memorySize);

    /**** SET UP REGISTER INTERFACE ****/
    this.vRegisters = new Uint8Array(memory.buffer, this.chip8.get_v_registers(), 16);
  }

  updateDisplay() {
    const imageData = this.ctx.createImageData(this.displayWidth, this.displayHeight);
    for (let i = 0; i < this.displayMemory.length; i++) {
      imageData.data[i * 4] = this.displayMemory[i] === 1 ? 0xff : 0;
      imageData.data[i * 4 + 1] = this.displayMemory[i] === 1 ? 0xff : 0;
      imageData.data[i * 4 + 2] = this.displayMemory[i] === 1 ? 0xff : 0;
      imageData.data[i * 4 + 3] = 0xff;
    }
    this.ctx.putImageData(imageData, 0, 0);
  }

  clearDisplay() {
    for (var i = 0; i < this.displayWidth * this.displayHeight; i++) {
      this.displayMemory[i] = 0;
    }
  }

  clearMemory() {
    for (var i = 0x200; i < this.memorySize; i++) {
      this.mainMemory[i] = 0;
    }
  }

  clearRegisters() {
    for (var i = 0; i < 16; i++) {
      this.vRegisters[i] = 0;
    }
    this.chip8.clear_control_registers();
  }

  getOpcodeFromMemory(pc) {
    return (this.mainMemory[pc] << 8) | this.mainMemory[pc + 1];
  }

  dissassembleOpcode(opcode) {
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
  }
}

/**** OUTPUT FUNCTIONS ****/
const writeProgramMemory = (emulator, length) => {
  var memory = "";
  const startpos = 0x200;
  for (var i = 0; i <= length; i = i + 2) {
    var opcode = emulator.getOpcodeFromMemory(startpos + i);
    memory += `<div class="program-listing-line" id="mem_${hex(startpos + i, 4)}">[${hex(startpos + i, 4)}]: ${emulator.dissassembleOpcode(opcode)} (${hex(
      opcode,
      4
    )})</div>`;
  }

  $("#memory #program-listing").html(memory);
};

const highlightCurrentOpcode = (emulator) => {
  var memoryElements = document.getElementsByClassName("program-listing-line");
  for (var i = 0; i < memoryElements.length; i++) {
    memoryElements[i].style.background = backgroundColour;
    memoryElements[i].style.color = highlightColour;
  }

  document.getElementById(`mem_${hex(emulator.chip8.get_pc(), 4)}`).style.background = highlightColour;
  document.getElementById(`mem_${hex(emulator.chip8.get_pc(), 4)}`).style.color = backgroundColour;
};

const writeRegisters = (emulator) => {
  var registers = "";

  for (var i = 0; i < 16; i++) {
    registers += `V${hex(i, 1)}: ${hex(emulator.vRegisters[i])}<br/>`;
  }

  registers += `PC: ${hex(emulator.chip8.get_pc(), 4)}<br/>`;
  registers += ` I: ${hex(emulator.chip8.get_i(), 4)}<br/>`;
  registers += `DT: ${hex(emulator.chip8.get_delay_timer())}<br/>`;
  registers += `ST: ${hex(emulator.chip8.get_sound_timer())}<br/>`;

  $("#memory #registers").html(registers);
};

/**** BUTTONS ****/
var is_step_through = false;
var is_running = false;
document.getElementById("step").onclick = function () {
  is_step_through = document.getElementById("step").checked;
  if (is_step_through) {
    document.getElementById("go_button").value = "Step";
  } else {
    document.getElementById("go_button").value = "Run";
  }
};

document.getElementById("go_button").onclick = function () {
  if (is_step_through) {
    renderLoop();
  } else {
    if (!is_running) {
      is_running = true;
      renderLoop();
      document.getElementById("go_button").value = "Stop";
    } else {
      is_running = false;
      document.getElementById("go_button").value = "Run";
    }
  }
};

const resetEmulator = () => {
  is_step_through = false;
  is_running = false;

  document.getElementById("go_button").value = "Run";

  em.clearDisplay();
  em.clearRegisters();
  em.clearMemory();
}

document.getElementById("reset_button").onclick = function () {
  loadRom(document.getElementById("roms").value);
  highlightCurrentOpcode(em);
  writeRegisters(em);
  em.updateDisplay();
};

window.hideOutput = function (id) {
  $(`#${id}`).toggle();
  if ($(`#hide-button-${id}`).val().localeCompare("+") == 0) {
    $(`#hide-button-${id}`).val("-");
  } else {
    $(`#hide-button-${id}`).val("+");
  }
};

const loadRom = (rom) => {
  fetch(`roms/${rom}`)
    .then(romData => romData.arrayBuffer())
    .then(romDataArrayBuffer => {
      resetEmulator();
      const romDataView = new DataView(romDataArrayBuffer, 0, romDataArrayBuffer.byteLength);
      for (var i = 0; i < romDataView.byteLength; i++) {
        em.mainMemory[0x200 + i] = romDataView.getUint8(i);
      }
    })
    .then(_ => {
      writeProgramMemory(em, 256);
      highlightCurrentOpcode(em);
      writeRegisters(em);
      em.updateDisplay();
    });
}

document.getElementById("roms").addEventListener("change", e => {
  loadRom(e.target.value);
});

document.getElementById("roms").value = "PONG";


/**** EMULATION LOOP ****/
function renderLoop() {
  if (is_step_through) {
    //if we're stepping through, only execute one cycle every frame
    console.log(hex(em.chip8.get_pc(), 4) + "   " + hex(em.getOpcodeFromMemory(em.chip8.get_pc()), 4));
    em.chip8.execute_cycle();
    em.chip8.decrement_timers();
  } else if (is_running) {
    //otherwise, execute 8 cycles on every frame (We want to run the emulation at close to 500Hz,
    // which is the normal operating clockspeed of the chip8. Since requestAnimationFrame() runs at
    // 60fps, executing 8 cycles per frame will give us a clockspeed of 480Hz).
    for (var i = 0; i < 8; i++) {
      em.chip8.execute_cycle();
      if (!is_running)
        break;
    }
    if (is_running)
      em.chip8.decrement_timers();
  }

  highlightCurrentOpcode(em);
  writeRegisters(em);
  em.updateDisplay();

  if (is_running && !is_step_through) {
    requestAnimationFrame(renderLoop);
  }
}

var em = new Emulator();
loadRom("PONG");


