import { Core } from "./emu/core.js"
import { Disassembler } from "./emu/cpu_dis.js"


let interval = null
let keyA = false
let keyB = false
let keySelect = false
let keyStart = false
let keyUp = false
let keyDown = false
let keyLeft = false
let keyRight = false


window.main = function()
{
	window.onkeydown = (ev) => handleKey(ev, true)
	window.onkeyup = (ev) => handleKey(ev, false)
	
	let canvas = document.getElementById("canvasScreen")
	
	let ctx = canvas.getContext("2d")
	ctx.fillStyle = "black"
	ctx.fillRect(0, 0, 256, 240)
	
	let inputFile = document.getElementById("inputFile")
	inputFile.onchange = () =>
	{
		if (inputFile.files.length != 1)
			return
		
		let reader = new FileReader()
		reader.onload = () => loadINES(reader.result)
		reader.readAsArrayBuffer(inputFile.files[0])
	}
}


function handleKey(ev, down)
{
	switch (ev.code)
	{
		case "Space": keyA = down; break
		case "KeyX": keyB = down; break
		case "ControlLeft": keySelect = down; break
		case "Enter": keyStart = down; break
		case "ArrowUp": keyUp = down; break
		case "ArrowDown": keyDown = down; break
		case "ArrowLeft": keyLeft = down; break
		case "ArrowRight": keyRight = down; break
		default: return
	}
	
	ev.preventDefault()
}


function loadINES(buffer)
{
	if (interval != null)
	{
		window.clearInterval(interval)
		interval = null
	}
	
	let emu = new Core()
	emu.loadINES(new Uint8Array(buffer))
	emu.reset()
	
	let canvas = document.getElementById("canvasScreen")
	let ctx = canvas.getContext("2d")
	let ctxData = ctx.createImageData(256, 240)
	emu.connect(
		(scanline, dot, color, mask) => output(emu, ctx, ctxData, scanline, dot, color, mask),
		(i) => [keyA, keyB, keySelect, keyStart, keyUp, keyDown, keyLeft, keyRight])
	
	console.log(emu)
	
	interval = window.setInterval(() =>
	{
		for (let i = 0; i < 28000; i++)
			emu.run()
		
	}, 1000 / 60)
	
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


function output(emu, ctx, ctxData, scanline, dot, color, mask)
{
	if (scanline == 0 && dot == 0)
		ctx.putImageData(ctxData, 0, 0)
	
	const dataAddr = ((scanline * 256) + dot) * 4
	const palAddr = color * 3
	ctxData.data[dataAddr + 0] = palette[palAddr + 0]
	ctxData.data[dataAddr + 1] = palette[palAddr + 1]
	ctxData.data[dataAddr + 2] = palette[palAddr + 2]
	ctxData.data[dataAddr + 3] = 255
}


const palette =
[
	0x75, 0x75, 0x75,
	0x27, 0x1b, 0x8f,
	0x00, 0x00, 0xab,
	0x47, 0x00, 0x9f,
	0x8f, 0x00, 0x77,
	0xab, 0x00, 0x13,
	0xa7, 0x00, 0x00,
	0x7f, 0x0b, 0x00,
	0x43, 0x2f, 0x00,
	0x00, 0x47, 0x00,
	0x00, 0x51, 0x00,
	0x00, 0x3f, 0x17,
	0x1b, 0x3f, 0x5f,
	0x00, 0x00, 0x00,
	0x00, 0x00, 0x00,
	0x00, 0x00, 0x00,
	
	0xbc, 0xbc, 0xbc,
	0x00, 0x73, 0xef,
	0x23, 0x3b, 0xef,
	0x83, 0x00, 0xf3,
	0xbf, 0x00, 0xbf,
	0xe7, 0x00, 0x5b,
	0xdb, 0x2b, 0x00,
	0xcb, 0x4f, 0x0f,
	0x8b, 0x73, 0x00,
	0x00, 0x97, 0x00,
	0x00, 0xab, 0x00,
	0x00, 0x93, 0x3b,
	0x00, 0x83, 0x8b,
	0x00, 0x00, 0x00,
	0x00, 0x00, 0x00,
	0x00, 0x00, 0x00,
	
	0xff, 0xff, 0xff,
	0x3f, 0xbf, 0xff,
	0x5f, 0x97, 0xff,
	0xa7, 0x8b, 0xfd,
	0xf7, 0x7b, 0xff,
	0xff, 0x77, 0xb7,
	0xff, 0x77, 0x63,
	0xff, 0x9b, 0x3b,
	0xf3, 0xbf, 0x3f,
	0x83, 0xd3, 0x13,
	0x4f, 0xdf, 0x4b,
	0x58, 0xf8, 0x98,
	0x00, 0xeb, 0xdb,
	0x00, 0x00, 0x00,
	0x00, 0x00, 0x00,
	0x00, 0x00, 0x00,
	
	0xff, 0xff, 0xff,
	0xab, 0xe7, 0xff,
	0xc7, 0xd7, 0xff,
	0xd7, 0xcb, 0xff,
	0xff, 0xc7, 0xff,
	0xff, 0xc7, 0xdb,
	0xff, 0xbf, 0xb3,
	0xff, 0xdb, 0xab,
	0xff, 0xe7, 0xa3,
	0xe3, 0xff, 0xa3,
	0xab, 0xf3, 0xbf,
	0xb3, 0xff, 0xcf,
	0x9f, 0xff, 0xf3,
	0x00, 0x00, 0x00,
	0x00, 0x00, 0x00,
	0x00, 0x00, 0x00
]