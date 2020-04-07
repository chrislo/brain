s.waitForBoot({
~b1 = Buffer.read(s, "/Users/chrislowis/src/chrislo/rust-sampler/1.wav");

SynthDef(\sampler, {
        arg obs=0, buf, rate=1, amp=1;
        var sig;
        sig = PlayBuf.ar(1, buf, rate, \t_tr.kr(1, 0), doneAction: Done.freeSelf);
        Out.ar(obs, sig);
}).add;

~handler = {
        arg msg, time, addr, recvPort;

        Synth(\sampler, [\buf, ~b1.bufnum]);
};

n = NetAddr.new("127.0.0.1");
o = OSCFunc(~handler, '/sampler/1', n, 49162);
});