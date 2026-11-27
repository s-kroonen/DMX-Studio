// WebGL2 side of the DMX-9 Avalonia browser fallback: input in, draw calls out. The scene is built
// in C# (WebRoom.cs) and arrives as a float32 vertex buffer each frame. Throwaway code.

let gl, program, vao, vbo, mvpLoc, canvas, tick;
let dx = 0, dy = 0, wheel = 0, clickX = -1, clickY = -1;

const vs = `#version 300 es
layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec4 a_col;
uniform mat4 u_mvp;
out vec4 v_col;
void main() { v_col = a_col; gl_Position = u_mvp * vec4(a_pos, 1.0); }`;
const fs = `#version 300 es
precision mediump float;
in vec4 v_col;
out vec4 o_col;
void main() { o_col = v_col; }`;

function compile(kind, src) {
  const s = gl.createShader(kind);
  gl.shaderSource(s, src);
  gl.compileShader(s);
  if (!gl.getShaderParameter(s, gl.COMPILE_STATUS)) throw new Error(gl.getShaderInfoLog(s));
  gl.attachShader(program, s);
}

export function start(c) {
  canvas = c;
  gl = canvas.getContext("webgl2", { antialias: true, depth: true });
  program = gl.createProgram();
  compile(gl.VERTEX_SHADER, vs);
  compile(gl.FRAGMENT_SHADER, fs);
  gl.linkProgram(program);
  mvpLoc = gl.getUniformLocation(program, "u_mvp");
  vao = gl.createVertexArray();
  vbo = gl.createBuffer();
  gl.bindVertexArray(vao);
  gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
  gl.enableVertexAttribArray(0);
  gl.vertexAttribPointer(0, 3, gl.FLOAT, false, 28, 0);
  gl.enableVertexAttribArray(1);
  gl.vertexAttribPointer(1, 4, gl.FLOAT, false, 28, 12);
  gl.bindVertexArray(null);

  // Orbit from the button state of each move: Avalonia's input layer takes some down and up
  // events, so tracking them gives a drag with the button released.
  let moved = 0;
  canvas.addEventListener("pointermove", e => {
    if (e.buttons & 1) { dx += e.movementX; dy += e.movementY; moved += Math.abs(e.movementX) + Math.abs(e.movementY); }
  });
  canvas.addEventListener("pointerdown", () => { moved = 0; });
  canvas.addEventListener("click", e => { if (moved <= 4) { clickX = e.offsetX; clickY = e.offsetY; } moved = 0; });
  canvas.addEventListener("wheel", e => { wheel += e.deltaY; e.preventDefault(); }, { passive: false });

  globalThis.getDotnetRuntime(0).getAssemblyExports("Browser.dll").then(exports => {
    tick = exports.Dmx9.WebRoom.Tick;
    requestAnimationFrame(frame);
  });
}

function frame() {
  const w = canvas.clientWidth, h = canvas.clientHeight, s = globalThis.devicePixelRatio || 1;
  if (canvas.width !== Math.round(w * s) || canvas.height !== Math.round(h * s)) {
    canvas.width = Math.round(w * s);
    canvas.height = Math.round(h * s);
  }
  const input = [dx, dy, wheel, clickX, clickY];
  dx = dy = wheel = 0; clickX = clickY = -1;
  tick(w, h, ...input);
  requestAnimationFrame(frame);
}

export function draw(view, opaque, mvp) {
  const verts = new Float32Array(view.slice().buffer);
  gl.viewport(0, 0, canvas.width, canvas.height);
  gl.clearColor(0.05, 0.05, 0.07, 1);
  gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
  gl.useProgram(program);
  gl.uniformMatrix4fv(mvpLoc, false, new Float32Array(mvp));
  gl.bindVertexArray(vao);
  gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
  gl.bufferData(gl.ARRAY_BUFFER, verts, gl.STREAM_DRAW);
  gl.enable(gl.DEPTH_TEST);
  gl.depthFunc(gl.LEQUAL);
  gl.disable(gl.BLEND);
  gl.drawArrays(gl.TRIANGLES, 0, opaque);
  gl.enable(gl.BLEND);
  gl.blendFunc(gl.SRC_ALPHA, gl.ONE);
  gl.depthMask(false);
  gl.drawArrays(gl.TRIANGLES, opaque, verts.length / 7 - opaque);
  gl.depthMask(true);
}
