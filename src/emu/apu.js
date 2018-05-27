export class APU
{
	constructor()
	{
		this.GLOBAL_VOLUME = 0.2
		this.AUDIO_DELAY = 0.2
		
		this.clock = 0
		
		this.regSTATUS = 0
		this.regFrameCounter = 0
		
		this.regPulse1DutyVolume = 0
		this.regPulse1Sweep = 0
		this.regPulse1TimerLow = 0
		this.regPulse1TimerHigh = 0
		
		this.regPulse2DutyVolume = 0
		this.regPulse2Sweep = 0
		this.regPulse2TimerLow = 0
		this.regPulse2TimerHigh = 0
		
		this.regTriangleLinearCounter = 0
		this.regTriangleTimerLow = 0
		this.regTriangleTimerHigh = 0
		
		this.regNoiseVolume = 0
		this.regNoiseTimer = 0
		this.regNoiseLengthCounter = 0
		
		this.pulse1Period = 0
		this.pulse1LengthCounter = 0
		this.pulse1EnvelopeDivider = 0
		this.pulse1EnvelopeReload = false
		this.pulse1EnvelopeDecayLevel = 0
		this.pulse1SweepReload = false
		this.pulse1SweepDivider = 0
		
		this.pulse2Period = 0
		this.pulse2LengthCounter = 0
		this.pulse2EnvelopeDivider = 0
		this.pulse2EnvelopeReload = false
		this.pulse2EnvelopeDecayLevel = 0
		this.pulse2SweepReload = false
		this.pulse2SweepDivider = 0
		
		this.trianglePeriod = 0
		this.triangleLengthCounter = 0
		this.triangleLinearCounter = 0
		this.triangleLinearCounterReload = false
		
		this.noisePeriod = 4
		this.noiseLengthCounter = 0
		this.noiseEnvelopeDivider = 0
		this.noiseEnvelopeReload = false
		this.noiseEnvelopeDecayLevel = 0
		
		this.lengthCounterTable =
		[
			10, 254, 20,  2, 40,  4, 80,  6, 160,  8, 60, 10, 14, 12, 26, 14,
			12,  16, 24, 18, 48, 20, 96, 22, 192, 24, 72, 26, 16, 28, 32, 30
		]
		
		this.noisePeriodTable =
		[
			4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068
		]
		
		this.frameCounterDivider = 0
		this.frameCounterStep = 0
		
		this.audioCtx = null
		this.audioCtxTimeAtBufferStart = 0
		this.audioEmulationSpeed = 1
		
		this.audioPulse1Wave = null
		this.audioPulse1Gain = null
		
		this.audioPulse2Wave = null
		this.audioPulse2Gain = null
		
		this.audioTriangleWave = null
		this.audioTriangleGain = null
		
		this.audioNoiseMode0Wave = null
		this.audioNoiseMode1Wave = null
		this.audioNoiseMode0Gain = null
		this.audioNoiseMode1Gain = null
		
		this.audioPulse1LastEnabled = false
		this.audioPulse1LastFreq = 0
		this.audioPulse1LastVolume = 0
		this.audioPulse1LastDutyCycle = 2
		
		this.audioPulse2LastEnabled = false
		this.audioPulse2LastFreq = 0
		this.audioPulse2LastVolume = 0
		this.audioPulse2LastDutyCycle = 2
		
		this.audioTriangleLastEnabled = false
		this.audioTriangleLastFreq = 0
		this.audioTriangleLastVolume = 0
		
		this.audioNoiseLastEnabled = false
		this.audioNoiseLastFreq = 0
		this.audioNoiseLastMode = 0
		this.audioNoiseLastVolume = 0
		
		this.audioPulseWaveforms = null
		this.audioNoiseWaveforms = null
	}
	
	
	connect(audioCtx)
	{
		this.audioCtx = audioCtx
		
		if (this.audioCtx == null)
			return
		
		this.audioCtxTimeAtBufferStart = this.audioCtx.currentTime
		this.audioEmulationSpeed = 1
		
		this.audioPulseWaveforms = []
		const zeroedHarmonicMultiples = [8, 4, 2, 8]
		for (let i = 0; i < 4; i++)
		{
			let realParts = [0]
			let imagParts = [0]
			for (let j = 1; j < 64; j++)
			{
				realParts.push((j % zeroedHarmonicMultiples[i] == 0) ? 0 : (1 / j))
				imagParts.push(0)
			}
			
			this.audioPulseWaveforms.push(this.audioCtx.createPeriodicWave(realParts, imagParts))
		}
		
		this.audioNoiseWaveforms = []
		const noiseSampleRate = 29780
		const noiseFeedbackBitIndex = [1, 6]
		const noiseLength = [32767, 93]
		for (let i = 0; i < 2; i++)
		{
			let buffer = this.audioCtx.createBuffer(1, noiseLength[i], noiseSampleRate)
			let bufferChannel = buffer.getChannelData(0)
			
			let shiftRegister = 1
			
			for (let j = 0; j < noiseLength[i]; j++)
			{
				bufferChannel[j] = ((shiftRegister & 1) != 0) ? 1 : -1
				
				let feedback = (shiftRegister & 1) ^ ((shiftRegister >> noiseFeedbackBitIndex[i]) & 1)
				shiftRegister = (shiftRegister >> 1) | (feedback << 14)
			}
			
			this.audioNoiseWaveforms.push(buffer)
		}
		
		this.audioPulse1Wave = this.audioCtx.createOscillator()
		this.audioPulse1Wave.setPeriodicWave(this.audioPulseWaveforms[2])
		this.audioPulse1Wave.frequency.setValueAtTime(0, this.audioCtx.currentTime)
		this.audioPulse1Wave.start()
		
		this.audioPulse1Gain = this.audioCtx.createGain()
		this.audioPulse1Gain.gain.setValueAtTime(0, this.audioCtx.currentTime)
		
		this.audioPulse1Wave.connect(this.audioPulse1Gain)
		this.audioPulse1Gain.connect(this.audioCtx.destination)
		
		this.audioPulse2Wave = this.audioCtx.createOscillator()
		this.audioPulse2Wave.setPeriodicWave(this.audioPulseWaveforms[2])
		this.audioPulse2Wave.frequency.setValueAtTime(0, this.audioCtx.currentTime)
		this.audioPulse2Wave.start()
		
		this.audioPulse2Gain = this.audioCtx.createGain()
		this.audioPulse2Gain.gain.setValueAtTime(0, this.audioCtx.currentTime)
		
		this.audioPulse2Wave.connect(this.audioPulse2Gain)
		this.audioPulse2Gain.connect(this.audioCtx.destination)
		
		this.audioTriangleWave = this.audioCtx.createOscillator()
		this.audioTriangleWave.type = "triangle"
		this.audioTriangleWave.frequency.setValueAtTime(0, this.audioCtx.currentTime)
		this.audioTriangleWave.start()
		
		this.audioTriangleGain = this.audioCtx.createGain()
		this.audioTriangleGain.gain.setValueAtTime(0, this.audioCtx.currentTime)
		
		this.audioTriangleWave.connect(this.audioTriangleGain)
		this.audioTriangleGain.connect(this.audioCtx.destination)
		
		this.audioNoiseMode0Wave = this.audioCtx.createBufferSource()
		this.audioNoiseMode0Wave.buffer = this.audioNoiseWaveforms[0]
		this.audioNoiseMode0Wave.loop = true
		this.audioNoiseMode0Wave.playbackRate.setValueAtTime(1, this.audioCtx.currentTime)
		this.audioNoiseMode0Wave.start()
		
		this.audioNoiseMode1Wave = this.audioCtx.createBufferSource()
		this.audioNoiseMode1Wave.buffer = this.audioNoiseWaveforms[1]
		this.audioNoiseMode1Wave.loop = true
		this.audioNoiseMode1Wave.playbackRate.setValueAtTime(1, this.audioCtx.currentTime)
		this.audioNoiseMode1Wave.start()
		
		this.audioNoiseMode0Gain = this.audioCtx.createGain()
		this.audioNoiseMode0Gain.gain.setValueAtTime(0, this.audioCtx.currentTime)
		
		this.audioNoiseMode1Gain = this.audioCtx.createGain()
		this.audioNoiseMode1Gain.gain.setValueAtTime(0, this.audioCtx.currentTime)
		
		this.audioNoiseMode0Wave.connect(this.audioNoiseMode0Gain)
		this.audioNoiseMode1Wave.connect(this.audioNoiseMode1Gain)
		this.audioNoiseMode0Gain.connect(this.audioCtx.destination)
		this.audioNoiseMode1Gain.connect(this.audioCtx.destination)
		
		this.audioPulse1LastEnabled = false
		this.audioPulse1LastFreq = 0
		this.audioPulse1LastVolume = 0
		this.audioPulse1LastDutyCycle = 2
		
		this.audioPulse2LastEnabled = false
		this.audioPulse2LastFreq = 0
		this.audioPulse2LastVolume = 0
		this.audioPulse2LastDutyCycle = 2
		
		this.audioTriangleLastEnabled = false
		this.audioTriangleLastFreq = 0
		this.audioTriangleLastVolume = 0
		
		this.audioNoiseLastEnabled = false
		this.audioNoiseLastFreq = 0
		this.audioNoiseLastMode = 0
		this.audioNoiseLastVolume = 0
	}
	
	
	reset()
	{
		this.clock = 0
	
		this.regSTATUS = 0
		this.regFrameCounter = 0
		
		this.regPulse1DutyVolume = 0
		this.regPulse1Sweep = 0
		this.regPulse1TimerLow = 0
		this.regPulse1TimerHigh = 0
		
		this.regPulse2DutyVolume = 0
		this.regPulse2Sweep = 0
		this.regPulse2TimerLow = 0
		this.regPulse2TimerHigh = 0
		
		this.regTriangleLinearCounter = 0
		this.regTriangleTimerLow = 0
		this.regTriangleTimerHigh = 0
		
		this.pulse1Period = 0
		this.pulse1LengthCounter = 0
		this.pulse1EnvelopeDivider = 0
		this.pulse1EnvelopeReload = false
		this.pulse1EnvelopeDecayLevel = 0
		this.pulse1SweepReload = false
		this.pulse1SweepDivider = 0
		
		this.pulse2Period = 0
		this.pulse2LengthCounter = 0
		this.pulse2EnvelopeDivider = 0
		this.pulse2EnvelopeReload = false
		this.pulse2EnvelopeDecayLevel = 0
		this.pulse2SweepReload = false
		this.pulse2SweepDivider = 0
		
		this.trianglePeriod = 0
		this.triangleLengthCounter = 0
		this.triangleLinearCounter = 0
		this.triangleLinearCounterReload = false
	}
	
	
	writeRegSTATUS(val)
	{
		this.regSTATUS = val
		
		if ((val & 0x1) == 0)
			this.pulse1LengthCounter = 0
		
		if ((val & 0x2) == 0)
			this.pulse2LengthCounter = 0
		
		if ((val & 0x4) == 0)
			this.triangleLengthCounter = 0
	}
	
	
	writeRegFrameCounter(val)
	{
		this.regFrameCounter = val
	}
	
	
	writeRegPulse1DutyVolume(val)
	{
		this.regPulse1DutyVolume = val
	}
	
	
	writeRegPulse1Sweep(val)
	{
		this.regPulse1Sweep = val
		this.pulse1SweepReload = true
	}
	
	
	writeRegPulse1TimerLow(val)
	{
		this.regPulse1TimerLow = val
		
		this.pulse1Period = (this.pulse1Period & ~0xff) | val
	}
	
	
	writeRegPulse1TimerHigh(val)
	{
		this.regPulse1TimerHigh = val
		
		this.pulse1Period = (this.pulse1Period & ~0xff00) | ((val & 0b111) << 8)
		this.pulse1EnvelopeReload = true
		
		if ((this.regSTATUS & 0x1) != 0)
			this.pulse1LengthCounter = this.lengthCounterTable[(val >> 3) & 0b11111]
	}
	
	
	writeRegPulse2DutyVolume(val)
	{
		this.regPulse2DutyVolume = val
	}
	
	
	writeRegPulse2Sweep(val)
	{
		this.regPulse2Sweep = val
		this.pulse2SweepReload = true
	}
	
	
	writeRegPulse2TimerLow(val)
	{
		this.regPulse2TimerLow = val
		
		this.pulse2Period = (this.pulse2Period & ~0xff) | val
	}
	
	
	writeRegPulse2TimerHigh(val)
	{
		this.regPulse2TimerHigh = val
		
		this.pulse2Period = (this.pulse2Period & ~0xff00) | ((val & 0b111) << 8)
		this.pulse2EnvelopeReload = true
		
		if ((this.regSTATUS & 0x2) != 0)
			this.pulse2LengthCounter = this.lengthCounterTable[(val >> 3) & 0b11111]
	}
	
	
	writeRegTriangleLinearCounter(val)
	{
		this.regTriangleLinearCounter = val
	}
	
	
	writeRegTriangleTimerLow(val)
	{
		this.regTriangleTimerLow = val
		
		this.trianglePeriod = (this.trianglePeriod & ~0xff) | val
	}
	
	
	writeRegTriangleTimerHigh(val)
	{
		this.regTriangleTimerHigh = val
		
		this.trianglePeriod = (this.trianglePeriod & ~0xff00) | ((val & 0b111) << 8)
		this.triangleLinearCounterReload = true
		
		if ((this.regSTATUS & 0x4) != 0)
			this.triangleLengthCounter = this.lengthCounterTable[(val >> 3) & 0b11111]
	}
	
	
	writeRegNoiseVolume(val)
	{
		this.regNoiseVolume = val
	}
	
	
	writeRegNoiseTimer(val)
	{
		this.regNoiseTimer = val
		
		this.noisePeriod = this.noisePeriodTable[val & 0xf]
	}
	
	
	writeRegNoiseLengthCounter(val)
	{
		this.regNoiseLengthCounter = val
		
		this.noiseEnvelopeReload = true
		
		if ((this.regSTATUS & 0x4) != 0)
			this.noiseLengthCounter = this.lengthCounterTable[(val >> 3) & 0b11111]
	}
	
	
	run()
	{
		this.clock += 1
		
		if (this.audioCtx == null)
			return
		
		const CLOCKS_PER_SECOND = 29780 * 60
		const CLOCKS_PER_FRAMECOUNTER_TICK = 29780 / 4
		
		if (this.clock == CLOCKS_PER_SECOND)
		{
			this.audioEmulationSpeed = (this.audioCtx.currentTime - this.audioCtxTimeAtBufferStart)
			this.audioCtxTimeAtBufferStart = this.audioCtx.currentTime
			this.clock = 0
			//console.log((this.audioCtxTimeAtBufferStart + this.AUDIO_DELAY).toFixed(5) + " : ----------- BEGIN BUFFER -----------")
		}
		
		const pulse1SweepTargetPeriod = this.pulse1Period +
			(((this.regPulse1Sweep & 0x8) != 0) ? -1 : 1) *
			(this.pulse1Period >> (this.regPulse1Sweep & 0b111))
		
		const pulse2SweepTargetPeriod = this.pulse2Period +
			(((this.regPulse2Sweep & 0x8) != 0) ? -1 : 0) +
			(((this.regPulse2Sweep & 0x8) != 0) ? -1 : 1) *
			(this.pulse2Period >> (this.regPulse2Sweep & 0b111))
			
		const pulse1SweepTargetPeriodInRange =
			(pulse1SweepTargetPeriod <= 0x7ff)
		
		const pulse2SweepTargetPeriodInRange =
			(pulse2SweepTargetPeriod <= 0x7ff)
			
		this.frameCounterDivider -= 1
		if (this.frameCounterDivider <= 0)
		{
			this.frameCounterDivider = CLOCKS_PER_FRAMECOUNTER_TICK
			this.frameCounterStep = (this.frameCounterStep + 1) % ((this.regFrameCounter & 0x80) != 0 ? 5 : 4)
			
			if (this.frameCounterStep <= 3)
			{
				if (!this.pulse1EnvelopeReload)
				{
					if (this.pulse1EnvelopeDivider == 0)
					{
						this.pulse1EnvelopeDivider = this.regPulse1DutyVolume & 0b1111
						if (this.pulse1EnvelopeDecayLevel > 0)
							this.pulse1EnvelopeDecayLevel -= 1
						else if ((this.regPulse1DutyVolume & 0x20) != 0)
							this.pulse1EnvelopeDecayLevel = 15
					}
					else
						this.pulse1EnvelopeDivider -= 1
				}
				else
				{
					this.pulse1EnvelopeReload = false
					this.pulse1EnvelopeDecayLevel = 15
					this.pulse1EnvelopeDivider = this.regPulse1DutyVolume & 0b1111
				}
				
				if (!this.pulse2EnvelopeReload)
				{
					if (this.pulse2EnvelopeDivider == 0)
					{
						this.pulse2EnvelopeDivider = this.regPulse2DutyVolume & 0b1111
						if (this.pulse2EnvelopeDecayLevel > 0)
							this.pulse2EnvelopeDecayLevel -= 1
						else if ((this.regPulse2DutyVolume & 0x20) != 0)
							this.pulse2EnvelopeDecayLevel = 15
					}
					else
						this.pulse2EnvelopeDivider -= 1
				}
				else
				{
					this.pulse2EnvelopeReload = false
					this.pulse2EnvelopeDecayLevel = 15
					this.pulse2EnvelopeDivider = this.regPulse2DutyVolume & 0b1111
				}
				
				if (!this.noiseEnvelopeReload)
				{
					if (this.noiseEnvelopeDivider == 0)
					{
						this.noiseEnvelopeDivider = this.regNoiseVolume & 0b1111
						if (this.noiseEnvelopeDecayLevel > 0)
							this.noiseEnvelopeDecayLevel -= 1
						else if ((this.regPulse2DutyVolume & 0x20) != 0)
							this.noiseEnvelopeDecayLevel = 15
					}
					else
						this.noiseEnvelopeDivider -= 1
				}
				else
				{
					this.noiseEnvelopeReload = false
					this.noiseEnvelopeDecayLevel = 15
					this.noiseEnvelopeDivider = this.regNoiseVolume & 0b1111
				}
				
				if (this.triangleLinearCounterReload)
					this.triangleLinearCounter = this.regTriangleLinearCounter & 0x7f
				else if (this.triangleLinearCounter > 0)
					this.triangleLinearCounter -= 1
				
				if ((this.regTriangleLinearCounter & 0x80) == 0)
					this.triangleLinearCounterReload = false
			}
			
			if (this.frameCounterStep == 1 || this.frameCounterStep == 3)
			{
				if (this.pulse1LengthCounter > 0 && (this.regPulse1DutyVolume & 0x20) == 0)
					this.pulse1LengthCounter -= 1
				
				if (this.pulse2LengthCounter > 0 && (this.regPulse2DutyVolume & 0x20) == 0)
					this.pulse2LengthCounter -= 1
				
				if (this.triangleLengthCounter > 0 && (this.regTriangleLinearCounter & 0x80) == 0)
					this.triangleLengthCounter -= 1
				
				if (this.noiseLengthCounter > 0 && (this.regNoiseVolume & 0x20) == 0)
					this.noiseLengthCounter -= 1
				
				if (this.pulse1SweepDivider == 0 && pulse1SweepTargetPeriodInRange && (this.regPulse1Sweep & 0x80) != 0)
					this.pulse1Period = pulse1SweepTargetPeriod
				
				if (this.pulse1SweepDivider == 0 || this.pulse1SweepReload)
				{
					this.pulse1SweepDivider = ((this.regPulse1Sweep >> 4) & 0b111)
					this.pulse1SweepReload = false
				}
				else
					this.pulse1SweepDivider -= 1
				
				if (this.pulse2SweepDivider == 0 && pulse2SweepTargetPeriodInRange && (this.regPulse2Sweep & 0x80) != 0)
					this.pulse2Period = pulse2SweepTargetPeriod
				
				if (this.pulse2SweepDivider == 0 || this.pulse2SweepReload)
				{
					this.pulse2SweepDivider = ((this.regPulse2Sweep >> 4) & 0b111)
					this.pulse2SweepReload = false
				}
				else
					this.pulse2SweepDivider -= 1
			}
		}
		
		const audioCtxTime = this.audioCtxTimeAtBufferStart + this.AUDIO_DELAY + (this.clock / CLOCKS_PER_SECOND) * this.audioEmulationSpeed
		
		const pulse1Freq = 1789773 / (16 * (this.pulse1Period + 1))
		const pulse1Volume = ((this.regPulse1DutyVolume & 0x10) == 0 ? this.pulse1EnvelopeDecayLevel : (this.regPulse1DutyVolume & 0b1111)) / 15
		const pulse1DutyCycle = (this.regPulse1DutyVolume >> 6) & 0b11
		const pulse1Enabled =
			this.pulse1Period >= 8 &&
			(this.pulse1LengthCounter > 0) &&
			pulse1SweepTargetPeriodInRange &&
			((this.regSTATUS & 0x1) != 0)
		
		if (this.audioPulse1LastFreq != pulse1Freq)
		{
			//console.log(audioCtxTime.toFixed(5) + " : pulse 1 freq = " + pulse1Freq)
			this.audioPulse1LastFreq = pulse1Freq
			this.audioPulse1Wave.frequency.setValueAtTime(Math.min(24000, pulse1Freq), audioCtxTime)
		}
		
		if (this.audioPulse1LastDutyCycle != pulse1DutyCycle)
		{
			// TODO: Split different duty cycle waveforms into different audio sources
			this.audioPulse1LastDutyCycle = pulse1DutyCycle
			this.audioPulse1Wave.setPeriodicWave(this.audioPulseWaveforms[pulse1DutyCycle])
		}
		
		if (this.audioPulse1LastEnabled != pulse1Enabled || this.audioPulse1LastVolume != pulse1Volume)
		{
			this.audioPulse1LastEnabled = pulse1Enabled
			this.audioPulse1LastVolume = pulse1Volume
			this.audioPulse1Gain.gain.setValueAtTime(pulse1Enabled ? this.GLOBAL_VOLUME * pulse1Volume : 0, audioCtxTime)
		}
		
		const pulse2Freq = 1789773 / (16 * (this.pulse2Period + 1))
		const pulse2Volume = ((this.regPulse2DutyVolume & 0x10) == 0 ? this.pulse2EnvelopeDecayLevel : (this.regPulse2DutyVolume & 0b1111)) / 15
		const pulse2DutyCycle = (this.regPulse2DutyVolume >> 6) & 0b11
		const pulse2Enabled =
			this.pulse2Period >= 8 &&
			(this.pulse2LengthCounter > 0) &&
			pulse2SweepTargetPeriodInRange &&
			((this.regSTATUS & 0x2) != 0)
			
		if (this.audioPulse2LastFreq != pulse2Freq)
		{
			this.audioPulse2LastFreq = pulse2Freq
			this.audioPulse2Wave.frequency.setValueAtTime(Math.min(24000, pulse2Freq), audioCtxTime)
		}
		
		if (this.audioPulse2LastDutyCycle != pulse2DutyCycle)
		{
			// TODO: Split different duty cycle waveforms into different audio sources
			this.audioPulse2LastDutyCycle = pulse2DutyCycle
			this.audioPulse2Wave.setPeriodicWave(this.audioPulseWaveforms[pulse2DutyCycle])
		}
		
		if (this.audioPulse2LastEnabled != pulse2Enabled || this.audioPulse2LastVolume != pulse2Volume)
		{
			this.audioPulse2LastEnabled = pulse2Enabled
			this.audioPulse2LastVolume = pulse2Volume
			this.audioPulse2Gain.gain.setValueAtTime(pulse2Enabled ? this.GLOBAL_VOLUME * pulse2Volume : 0, audioCtxTime)
		}
		
		const triangleFreq = 1789773 / (32 * (this.trianglePeriod + 1))
		const triangleVolume = 0.5
		const triangleEnabled =
			this.triangleLinearCounter > 0 &&
			(this.triangleLengthCounter > 0) &&
			((this.regSTATUS & 0x4) != 0)
		
		if (this.audioTriangleLastFreq != triangleFreq)
		{
			this.audioTriangleLastFreq = triangleFreq
			this.audioTriangleWave.frequency.setValueAtTime(Math.min(24000, triangleFreq), audioCtxTime)
		}
		
		if (this.audioTriangleLastEnabled != triangleEnabled || this.audioTriangleLastVolume != triangleVolume)
		{
			this.audioTriangleLastEnabled = triangleEnabled
			this.audioTriangleLastVolume = triangleVolume
			this.audioTriangleGain.gain.setValueAtTime(triangleEnabled ? this.GLOBAL_VOLUME * triangleVolume : 0, audioCtxTime)
		}
		
		const noiseVolume = ((this.regNoiseVolume & 0x10) == 0 ? this.noiseEnvelopeDecayLevel : (this.regNoiseVolume & 0b1111)) / 15 * 0.25
		const noiseMode = (this.regNoiseTimer & 0x80) != 0 ? 1 : 0
		const noiseEnabled =
			(this.noiseLengthCounter > 0) &&
			((this.regSTATUS & 0x8) != 0)
		
		if (this.audioNoiseLastPeriod != this.noisePeriod)
		{
			//console.log(audioCtxTime.toFixed(5) + " : noise period = " + this.noisePeriod + ", mode = " + noiseMode)
			this.audioNoiseLastPeriod = this.noisePeriod
			this.audioNoiseMode0Wave.playbackRate.setValueAtTime(256 / this.noisePeriod, audioCtxTime)
			this.audioNoiseMode1Wave.playbackRate.setValueAtTime(256 / this.noisePeriod, audioCtxTime)
		}
		
		if (this.audioNoiseLastEnabled != noiseEnabled || this.audioNoiseLastVolume != noiseVolume || this.audioNoiseLastMode != noiseMode)
		{
			this.audioNoiseLastEnabled = noiseEnabled
			this.audioNoiseLastVolume = noiseVolume
			this.audioNoiseLastMode = noiseMode
			this.audioNoiseMode0Gain.gain.setValueAtTime(noiseEnabled && noiseMode == 0 ? this.GLOBAL_VOLUME * noiseVolume : 0, audioCtxTime)
			this.audioNoiseMode1Gain.gain.setValueAtTime(noiseEnabled && noiseMode == 1 ? this.GLOBAL_VOLUME * noiseVolume : 0, audioCtxTime)
		}
		
		if (false && this.clock % (CLOCKS_PER_SECOND / 60) == 0)
			console.log("pulse1(" +
				this.pulse1Period + ", " +
				this.pulse1LengthCounter + ", " +
				this.pulse1EnvelopeDecayLevel + ", " +
				"0b" + this.regPulse1DutyVolume.toString(2) + ", " +
				(this.pulse1Period >= 8) + ", " +
				(this.pulse1LengthCounter > 0) + ", " +
				pulse1SweepTargetPeriodInRange + ", " +
				((this.regSTATUS & 0x1) != 0) + ") " +
				"pulse2(" +
				this.pulse2Period + ", " +
				this.pulse2LengthCounter + ", " +
				this.pulse2EnvelopeDecayLevel + ", " +
				"0b" + this.regPulse2DutyVolume.toString(2) + ", " +
				(this.pulse2Period >= 8) + ", " +
				(this.pulse2LengthCounter > 0) + ", " +
				pulse2SweepTargetPeriodInRange + ", " +
				((this.regSTATUS & 0x2) != 0) + ") " +
				"triangle(" +
				this.trianglePeriod + ", " +
				this.triangleLengthCounter + ", " +
				this.triangleLinearCounter + ", " +
				(this.triangleLinearCounter > 0) + ", " +
				(this.triangleLengthCounter > 0) + ", " +
				((this.regSTATUS & 0x4) != 0) + ")")
	}
}