<CsoundSynthesizer>
<CsOptions>

</CsOptions>
<CsInstruments>
sr      =  44100
ksmps   =  32
nchnls  =  2
0dbfs   =  1

chn_k "feedback", 1, 2, 0.6, 0, 1
chnset 0.6, "feedback"

chn_k "cut_off", 1, 2, 0.6, 0, 1
chnset 0.6, "cut_off"

chn_k "mix", 1, 2, 0.6, 0, 1
chnset 0.6, "mix"


        instr 1
kfeedback chngetk "feedback"
kcutOff chngetk "cut_off"
kmix chngetk "mix"
;        printks "kfeedback = %f, kcutoff = %f, kmix = %f\\n", 0.1, kfeedback, kcutOff, kmix
printk2 kfeedback

a1      vco2 0.85, 440, 10
kfrq    port 100, 0.004, 20000
a1      butterlp a1, kfrq
a2      linseg 0, 0.003, 1, 0.01, 0.7, 0.005, 0, 1, 0
a1      =  a1 * a2
a2      =  a1 * p5
a1      =  a1 * p4
        denorm a1, a2
aL, aR  reverbsc a1, a2, 0.85, 12000, sr, 0.5, 1
        outs a1 + aL, a2 + aR
        endin

</CsInstruments>
<CsScore>
f 0 36000
i 1 0 -1 0.71 0.71
e 
</CsScore>
</CsoundSynthesizer>
