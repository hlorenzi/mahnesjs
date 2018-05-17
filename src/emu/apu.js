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
		
		this.lengthCounterTable =
		[
			10, 254, 20,  2, 40,  4, 80,  6, 160,  8, 60, 10, 14, 12, 26, 14,
			12,  16, 24, 18, 48, 20, 96, 22, 192, 24, 72, 26, 16, 28, 32, 30
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
		
		this.audioPulseWaveforms = null
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