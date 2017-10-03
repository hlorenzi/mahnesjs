import { Core } from "./emu/core.js"
import { CPU } from "./emu/cpu.js"


window.main = function()
{
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


function loadINES(buffer)
{
	let emu = new Core()
	emu.loadINES(new Uint8Array(buffer))

	emu.cpu.reset()
	
	let clock = 0
	let buttonStep = document.getElementById("buttonStep")
	buttonStep.onclick = () =>
	{
		console.error("clock " + clock)
		console.log("opcode = " + emu.cpu.opcode.toString(16) + " (step " + emu.cpu.opcodeStep + ")")
		console.log("pc = " + emu.cpu.regPC.toString(16))
		console.log("stack = " + emu.cpu.regS.toString(16))
		console.log("A = " + emu.cpu.regA.toString(16))
		console.log("X = " + emu.cpu.regX.toString(16))
		console.log("Y = " + emu.cpu.regY.toString(16))
		emu.cpu.run()
		clock += 1
	}
}