(window.webpackJsonp=window.webpackJsonp||[]).push([[1],[,function(t,e,r){"use strict";r.r(e);var n=r(2),i=r(4),o=r.n(i),u=r(3);const s=getComputedStyle(document.body).backgroundColor,c=getComputedStyle(document.body).color;["15PUZZLE","BLINKY","BLITZ","BRIX","CONNECT4","GUESS","HIDDEN","IBM","INVADERS","KALEID","MAZE","MERLIN","MISSILE","PONG","PONG2","PUZZLE","SYZYGY","TANK","TETRIS","TICTAC","UFO","VBRIX","VERS","WIPEOFF"].forEach(t=>{o()("#roms").append(`<option value='${t}'>${t}</option>`)});const a=(t,e=2)=>{const r="0000"+t.toString(16).toUpperCase();return r.substr(r.length-e)},d=(t,e,r)=>t>=e&&t<=r;const l=t=>{for(var e=document.getElementsByClassName("program-listing-line"),r=0;r<e.length;r++)e[r].style.background=s,e[r].style.color=c;const n=document.getElementById("mem_"+a(t.chip8.get_pc(),4));n&&(n.style.background=c,n.style.color=s)},f=t=>{for(var e="",r=0;r<16;r++)e+=`V${a(r,1)}: ${a(t.vRegisters[r])}<br/>`;e+=`PC: ${a(t.chip8.get_pc(),4)}<br/>`,e+=` I: ${a(t.chip8.get_i(),4)}<br/>`,e+=`DT: ${a(t.chip8.get_delay_timer())}<br/>`,e+=`ST: ${a(t.chip8.get_sound_timer())}<br/>`,o()("#memory #registers").html(e)};var h=!1,p=!1;document.getElementById("step").onclick=function(){h=document.getElementById("step").checked,document.getElementById("go_button").value=h?"Step":"Run"},document.getElementById("go_button").onclick=function(){h?_():p?(p=!1,document.getElementById("go_button").value="Run"):(p=!0,_(),document.getElementById("go_button").value="Stop")};document.getElementById("reset_button").onclick=function(){m(document.getElementById("roms").value),l(v),f(v),v.updateDisplay()},window.hideOutput=function(t){o()("#"+t).toggle(),0==o()("#hide-button-"+t).val().localeCompare("+")?o()("#hide-button-"+t).val("-"):o()("#hide-button-"+t).val("+")};const m=t=>{fetch("roms/"+t).then(t=>t.arrayBuffer()).then(t=>{h&&(document.getElementById("step").checked=!1,h=!1),p=!1,document.getElementById("go_button").value="Run",v.clearDisplay(),v.clearRegisters(),v.clearMemory();const e=new DataView(t,0,t.byteLength);for(var r=0;r<e.byteLength;r++)v.mainMemory[512+r]=e.getUint8(r);((t,e)=>{var r="";for(var n=0;n<=e;n+=2){var i=t.getOpcodeFromMemory(512+n);r+=`<li class="program-listing-line" id="mem_${a(512+n,4)}">[${a(512+n,4)}]: ${t.dissassembleOpcode(i)} (${a(i,4)})</li>`}o()("#memory #program-listing").html(r)})(v,e.byteLength),l(v),f(v),v.updateDisplay()})};document.getElementById("roms").addEventListener("change",t=>{m(t.target.value),document.getElementById("roms").blur()}),document.getElementById("roms").value="PONG";var y={1:1,2:2,3:3,4:12,Q:4,W:5,E:6,R:13,A:7,S:8,D:9,F:14,Z:10,X:0,C:11,V:15,q:4,w:5,e:6,r:13,a:7,s:8,d:9,f:14,z:10,x:0,c:11,v:15};document.addEventListener("keydown",(function(t){y[t.key]&&v.chip8.press_key(y[t.key])})),document.addEventListener("keyup",(function(t){y[t.key]&&v.chip8.release_key(y[t.key])}));var g=new Audio("./assets/sound.wav");function _(){if(v.chip8.get_sound_timer()>0?g.play():(g.pause(),g.currentTime=0),h)v.chip8.execute_cycle(),v.chip8.decrement_timers();else if(p){for(var t=0;t<8&&(v.chip8.execute_cycle(),p);t++);p&&v.chip8.decrement_timers()}l(v),f(v),v.updateDisplay(),p&&!h&&requestAnimationFrame(_)}var v=new class{constructor(){this.chip8=n.a.power_up(),this.displayWidth=this.chip8.get_display_width(),this.displayHeight=this.chip8.get_display_height(),this.displayMemory=new Uint8Array(u.v.buffer,this.chip8.get_display_memory(),this.displayWidth*this.displayHeight),this.canvas=document.getElementById("chip8-display"),this.ctx=this.canvas.getContext("2d"),this.memorySize=this.chip8.get_memory_size(),this.mainMemory=new Uint8Array(u.v.buffer,this.chip8.get_memory(),this.memorySize),this.vRegisters=new Uint8Array(u.v.buffer,this.chip8.get_v_registers(),16)}updateDisplay(){const t=this.ctx.createImageData(this.displayWidth,this.displayHeight);for(let e=0;e<this.displayMemory.length;e++)t.data[4*e]=1===this.displayMemory[e]?255:0,t.data[4*e+1]=1===this.displayMemory[e]?255:0,t.data[4*e+2]=1===this.displayMemory[e]?255:0,t.data[4*e+3]=255;this.ctx.putImageData(t,0,0)}clearDisplay(){for(var t=0;t<this.displayWidth*this.displayHeight;t++)this.displayMemory[t]=0}clearMemory(){for(var t=512;t<this.memorySize;t++)this.mainMemory[t]=0}clearRegisters(){for(var t=0;t<16;t++)this.vRegisters[t]=0;this.chip8.clear_control_registers()}getOpcodeFromMemory(t){return this.mainMemory[t]<<8|this.mainMemory[t+1]}dissassembleOpcode(t){const e=(3840&t)>>8,r=(240&t)>>4,n=4095&t,i=255&t,o=15&t;if(224===t)return"CLS";if(238===t)return"RET";if(d(t,4096,8191))return"JP 0x"+a(n,3);if(d(t,8192,12287))return"CALL 0x"+a(n,3);if(d(t,12288,16383))return`SE V${o} ${a(i)}`;if(d(t,16384,20479))return`SNE V${o} ${a(i)}`;if(d(t,20480,24575))return`SE V${e} V${r}`;if(d(t,24576,28671))return`LD V${e} ${a(i)}`;if(d(t,28672,32767))return`ADD V${e} ${a(i)}`;if(d(t,32768,36863)){if(0===o)return`LD V${e} V${r}`;if(1===o)return`OR V${e} V${r}`;if(2===o)return`AND V${e} V${r}`;if(3===o)return`XOR V${e} V${r}`;if(4===o)return`ADD V${e} V${r}`;if(5===o)return`SUB V${e} V${r}`;if(6===o)return"SHR V"+e;if(7===o)return`SUBN V${e} V${r}`;if(14===o)return"SHL V"+e}if(d(t,36864,40959))return`SNE V${e} V${r}`;if(d(t,40960,45055))return"LDI "+a(n,3);if(d(t,45056,49151))return"JP V0 + "+a(n,3);if(d(t,49152,53247))return"RND "+a(i);if(d(t,53248,57343))return`DRW V${e} V${r} ${o}`;if(d(t,57344,61439)){if(158===i)return"SKP V"+e;if(161===i)return"SKNP V"+e}if(d(t,61440,65535)){if(7===i)return`LD V${e} DT`;if(10===i)return`LD V${e} K`;if(21===i)return"LD DT, V"+e;if(30===i)return"ADD I, V"+e;if(41===i)return"LD F, V"+e;if(51===i)return"LD B, V"+e;if(85===i)return"LD [I], "+e;if(101===i)return`LD ${e}, [I]`}return"-"}};m("PONG")},function(t,e,r){"use strict";(function(t){r.d(e,"a",(function(){return v})),r.d(e,"f",(function(){return E})),r.d(e,"j",(function(){return b})),r.d(e,"c",(function(){return $})),r.d(e,"l",(function(){return V})),r.d(e,"i",(function(){return D})),r.d(e,"h",(function(){return w})),r.d(e,"b",(function(){return S})),r.d(e,"k",(function(){return L})),r.d(e,"d",(function(){return B})),r.d(e,"e",(function(){return k})),r.d(e,"g",(function(){return R})),r.d(e,"m",(function(){return M}));var n=r(3);const i=new Array(32).fill(void 0);function o(t){return i[t]}i.push(void 0,null,!0,!1);let u=i.length;function s(t){const e=o(t);return function(t){t<36||(i[t]=u,u=t)}(t),e}let c=new("undefined"==typeof TextDecoder?(0,t.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});c.decode();let a=null;function d(){return null!==a&&a.buffer===n.v.buffer||(a=new Uint8Array(n.v.buffer)),a}function l(t,e){return c.decode(d().subarray(t,t+e))}function f(t){u===i.length&&i.push(i.length+1);const e=u;return u=i[e],i[e]=t,e}let h=0;let p=new("undefined"==typeof TextEncoder?(0,t.require)("util").TextEncoder:TextEncoder)("utf-8");const m="function"==typeof p.encodeInto?function(t,e){return p.encodeInto(t,e)}:function(t,e){const r=p.encode(t);return e.set(r),{read:t.length,written:r.length}};let y=null;function g(){return null!==y&&y.buffer===n.v.buffer||(y=new Int32Array(n.v.buffer)),y}function _(t,e){return d().subarray(t/1,t/1+e)}class v{static __wrap(t){const e=Object.create(v.prototype);return e.ptr=t,e}free(){const t=this.ptr;this.ptr=0,n.a(t)}static power_up(){var t=n.s();return v.__wrap(t)}execute_cycle(){n.h(this.ptr)}get_display_width(){return n.l(this.ptr)>>>0}get_display_height(){return n.j(this.ptr)>>>0}get_display_memory(){return n.k(this.ptr)}get_memory_size(){return n.o(this.ptr)>>>0}get_memory(){return n.n(this.ptr)}get_pc(){return n.p(this.ptr)}get_i(){return n.m(this.ptr)}get_delay_timer(){return n.i(this.ptr)}get_sound_timer(){return n.q(this.ptr)}clear_control_registers(){n.f(this.ptr)}decrement_timers(){n.g(this.ptr)}get_v_registers(){return n.r(this.ptr)}press_key(t){n.t(this.ptr,t)}release_key(t){n.u(this.ptr,t)}}const E=function(){return f(new Error)},b=function(t,e){var r=function(t,e,r){if(void 0===r){const r=p.encode(t),n=e(r.length);return d().subarray(n,n+r.length).set(r),h=r.length,n}let n=t.length,i=e(n);const o=d();let u=0;for(;u<n;u++){const e=t.charCodeAt(u);if(e>127)break;o[i+u]=e}if(u!==n){0!==u&&(t=t.slice(u)),i=r(i,n,n=u+3*t.length);const e=d().subarray(i+u,i+n);u+=m(t,e).written}return h=u,i}(o(e).stack,n.d,n.e),i=h;g()[t/4+1]=i,g()[t/4+0]=r},$=function(t,e){try{console.error(l(t,e))}finally{n.c(t,e)}},V=function(t){s(t)},D=(I=function(){return f(self.self)},function(){try{return I.apply(this,arguments)}catch(t){n.b(f(t))}});var I;const w=function(t,e){return f(r(6)(l(t,e)))},S=function(t){return f(o(t).crypto)},L=function(t){return void 0===o(t)},B=function(t){return f(o(t).getRandomValues)},k=function(t,e,r){o(t).getRandomValues(_(e,r))},R=function(t,e,r){o(t).randomFillSync(_(e,r))},M=function(t,e){throw new Error(l(t,e))}}).call(this,r(5)(t))},function(t,e,r){"use strict";var n=r.w[t.i];t.exports=n;r(2);n.w()},,,function(t,e){function r(t){var e=new Error("Cannot find module '"+t+"'");throw e.code="MODULE_NOT_FOUND",e}r.keys=function(){return[]},r.resolve=r,t.exports=r,r.id=6}]]);