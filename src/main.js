import { Core } from "./emu/core.js"
import { Disassembler } from "./emu/cpu_dis.js"


let g_wasm = null
let g_desiredBackend = 0
let g_isRunning = false

let audioCtx = null
let keyA = false
let keyB = false
let keySelect = false
let keyStart = false
let keyUp = false
let keyDown = false
let keyLeft = false
let keyRight = false


export function main()
{
	window.onkeydown = (ev) => handleKey(ev, true)
	window.onkeyup = (ev) => handleKey(ev, false)
	
	fetch("../mahnes_rs.gc.wasm")
		.then(r => r.arrayBuffer())
		.then(r => WebAssembly.instantiate(r))
		.then(wasm =>
		{
			g_wasm = wasm
			document.getElementById("radioRunWasm").disabled = false
			document.getElementById("labelRadioRunWasm").innerHTML = "Rust + WebAssembly"
		})
		
	let canvas = document.getElementById("canvasScreen")
	
	let ctx = canvas.getContext("2d")
	ctx.fillStyle = "black"
	ctx.fillRect(0, 0, 256, 240)
	
	document.getElementById("radioRunJS")  .onclick = () => handleRadioBackendOnChange(0)
	document.getElementById("radioRunWasm").onclick = () => handleRadioBackendOnChange(1)
	
	let inputFile = document.getElementById("inputFile")
	inputFile.onchange = () =>
	{
		if (inputFile.files.length != 1)
			return
		
		let reader = new FileReader()
		reader.onload = () => (g_desiredBackend == 0 ? loadJS(reader.result) : loadWasm(reader.result))
		reader.readAsArrayBuffer(inputFile.files[0])
	}
}


function handleRadioBackendOnChange(i)
{
	g_desiredBackend = i
}


function handleKey(ev, down)
{
	switch (ev.key)
	{
		case " ":
		case "Z":
		case "z":
			keyA = down
			break
			
		case "X":
		case "x":
			keyB = down
			break
			
		case "Control":
		case "Shift":
		case "G":
		case "g":
			keySelect = down
			break
			
		case "Enter":
		case "H":
		case "h":
			keyStart = down
			break
			
		case "ArrowUp":
		case "W":
		case "w":
			keyUp = down
			break
			
		case "ArrowDown":
		case "S":
		case "s":
			keyDown = down
			break
			
		case "ArrowLeft":
		case "A":
		case "a":
			keyLeft = down
			break
			
		case "ArrowRight":
		case "D":
		case "d":
			keyRight = down
			break
			
		default:
			return
	}
	
	ev.preventDefault()
}


function loadJS(buffer)
{
	let emu = new Core()
	
	try { emu.loadINES(new Uint8Array(buffer)) }
	catch (e) { alert(e); return }
	
	emu.reset()
	
	if (audioCtx != null)
		audioCtx.close()
	
	audioCtx = new AudioContext()
	
	let canvas = document.getElementById("canvasScreen")
	let ctx = canvas.getContext("2d")
	let ctxData = ctx.createImageData(256, 240)
	emu.connect(
		(scanline, dot, color, mask) => outputJS(emu, ctx, ctxData, scanline, dot, color, mask),
		(i) => [keyA, keyB, keySelect, keyStart, keyUp, keyDown, keyLeft, keyRight],
		audioCtx)
	
	console.log(emu)
	
	g_isRunning = true
	window.requestAnimationFrame(() => runFrameJS(emu))
	
	emu.cpu.hookExecuteInstruction = (addr, byte1, byte2, byte3) =>
	{
		/*console.log(addr.toString(16).padStart(4, "0") + ": " +
			"clock(" + emu.clock + ") " +
			"opcode(" + emu.cpu.opcode.toString(16) + ") " +
			"s(" + emu.cpu.regS.toString(16) + ") " +
			"a(" + emu.cpu.regA.toString(16) + ") " +
			"x(" + emu.cpu.regX.toString(16) + ") " +
			"y(" + emu.cpu.regY.toString(16) + ") " +
			"\t" +
			Disassembler.disassembleInstruction(addr, byte1, byte2, byte3));*/
	}
	
	document.getElementById("buttonDebug").onclick = () =>
	{
		let s = "mem:\n"
		for (let j = 0; j < 16; j++)
		{
			s += (0x600 + j * 16).toString(16).padStart(2, "0") + ": "
			
			for (let i = 0; i < 16; i++)
				s += emu.ram[0x600 + j * 16 + i].toString(16).padStart(2, "0") + " "
			
			s += "\n"
		}
		console.log(s)
		
		s = "first nametable:\n"
		for (let j = 0; j < 32; j++)
		{
			s += (j * 32).toString(16).padStart(2, "0") + ": "
			
			for (let i = 0; i < 32; i++)
				s += emu.vram[j * 32 + i].toString(16).padStart(2, "0") + " "
			
			s += "\n"
		}
		console.log(s)
		
		s = "palram:\n"
		for (let j = 0; j < 2; j++)
		{
			for (let i = 0; i < 16; i++)
				s += emu.palram[j * 16 + i].toString(16).padStart(2, "0") + " "
			
			s += "\n"
		}
		console.log(s)
	}
}


function loadWasm(buffer)
{
	let wasm_buffer = g_wasm.instance.exports.wasm_buffer_new(buffer.length)
	for (let i = 0; i < buffer.length; i++)
		g_wasm.instance.exports.wasm_buffer_set(wasm_buffer, i, buffer[i])
	
	g_wasm.instance.exports.wasm_core_new(wasm_buffer)
	g_wasm.instance.exports.wasm_buffer_drop(wasm_buffer)
		
	console.log("ok!")
}


function runFrameJS(emu)
{
	for (let i = 0; i < 29780; i++)
		emu.run()
	
	if (g_isRunning)
		window.requestAnimationFrame(() => runFrameJS(emu))
}


function runFrameWasm()
{
	for (let i = 0; i < 29780; i++)
		emu.run()
	
	if (g_isRunning)
		window.requestAnimationFrame(() => runFrameWasm())
}


function outputJS(emu, ctx, ctxData, scanline, dot, color, mask)
{
	if (scanline == 0 && dot == 0)
		ctx.putImageData(ctxData, 0, 0)
	
	const dataAddr = ((scanline * 256) + dot) * 4
	const palAddr = color * 4
	ctxData.data[dataAddr + 0] = palette[palAddr + 0]
	ctxData.data[dataAddr + 1] = palette[palAddr + 1]
	ctxData.data[dataAddr + 2] = palette[palAddr + 2]
	ctxData.data[dataAddr + 3] = palette[palAddr + 3]
}


const palette =
[
	0x75, 0x75, 0x75, 0xff,
	0x27, 0x1b, 0x8f, 0xff,
	0x00, 0x00, 0xab, 0xff,
	0x47, 0x00, 0x9f, 0xff,
	0x8f, 0x00, 0x77, 0xff,
	0xab, 0x00, 0x13, 0xff,
	0xa7, 0x00, 0x00, 0xff,
	0x7f, 0x0b, 0x00, 0xff,
	0x43, 0x2f, 0x00, 0xff,
	0x00, 0x47, 0x00, 0xff,
	0x00, 0x51, 0x00, 0xff,
	0x00, 0x3f, 0x17, 0xff,
	0x1b, 0x3f, 0x5f, 0xff,
	0x00, 0x00, 0x00, 0xff,
	0x00, 0x00, 0x00, 0xff,
	0x00, 0x00, 0x00, 0xff,
	
	0xbc, 0xbc, 0xbc, 0xff,
	0x00, 0x73, 0xef, 0xff,
	0x23, 0x3b, 0xef, 0xff,
	0x83, 0x00, 0xf3, 0xff,
	0xbf, 0x00, 0xbf, 0xff,
	0xe7, 0x00, 0x5b, 0xff,
	0xdb, 0x2b, 0x00, 0xff,
	0xcb, 0x4f, 0x0f, 0xff,
	0x8b, 0x73, 0x00, 0xff,
	0x00, 0x97, 0x00, 0xff,
	0x00, 0xab, 0x00, 0xff,
	0x00, 0x93, 0x3b, 0xff,
	0x00, 0x83, 0x8b, 0xff,
	0x00, 0x00, 0x00, 0xff,
	0x00, 0x00, 0x00, 0xff,
	0x00, 0x00, 0x00, 0xff,
	
	0xff, 0xff, 0xff, 0xff,
	0x3f, 0xbf, 0xff, 0xff,
	0x5f, 0x97, 0xff, 0xff,
	0xa7, 0x8b, 0xfd, 0xff,
	0xf7, 0x7b, 0xff, 0xff,
	0xff, 0x77, 0xb7, 0xff,
	0xff, 0x77, 0x63, 0xff,
	0xff, 0x9b, 0x3b, 0xff,
	0xf3, 0xbf, 0x3f, 0xff,
	0x83, 0xd3, 0x13, 0xff,
	0x4f, 0xdf, 0x4b, 0xff,
	0x58, 0xf8, 0x98, 0xff,
	0x00, 0xeb, 0xdb, 0xff,
	0x00, 0x00, 0x00, 0xff,
	0x00, 0x00, 0x00, 0xff,
	0x00, 0x00, 0x00, 0xff,
	
	0xff, 0xff, 0xff, 0xff,
	0xab, 0xe7, 0xff, 0xff,
	0xc7, 0xd7, 0xff, 0xff,
	0xd7, 0xcb, 0xff, 0xff,
	0xff, 0xc7, 0xff, 0xff,
	0xff, 0xc7, 0xdb, 0xff,
	0xff, 0xbf, 0xb3, 0xff,
	0xff, 0xdb, 0xab, 0xff,
	0xff, 0xe7, 0xa3, 0xff,
	0xe3, 0xff, 0xa3, 0xff,
	0xab, 0xf3, 0xbf, 0xff,
	0xb3, 0xff, 0xcf, 0xff,
	0x9f, 0xff, 0xf3, 0xff,
	0x00, 0x00, 0x00, 0xff,
	0x00, 0x00, 0x00, 0xff,
	0x00, 0x00, 0x00, 0xff,
]