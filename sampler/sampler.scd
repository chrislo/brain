s.waitForBoot({
  ~dict = Dictionary.new(n: 16);

  PathName.new("samples/").filesDo { |afile|
    var buffer;
    buffer = Buffer.read(s, afile.fullPath);
    ~dict.put(afile.fileNameWithoutExtension, buffer);
  };

  SynthDef(\playMono, {
    arg obs=0, buf, rate=1, amp=1;
    var sig, pan;
    sig = PlayBuf.ar(1, buf, rate, \t_tr.kr(1, 0), doneAction: Done.freeSelf);
    pan = Pan2.ar(sig, 0, 1);
    Out.ar(obs, pan);
  }).add;

  SynthDef(\playStereo, {
    arg obs=0, buf, rate=1, amp=1;
    var sig;
    sig = PlayBuf.ar(2, buf, rate, \t_tr.kr(1, 0), doneAction: Done.freeSelf);
    Out.ar(obs, sig);
  }).add;

  ~handler = {
    arg msg, time, addr, recvPort;
    var buffer, number;
    number = msg[1].asString;

    if (~dict.includesKey(number)) {
      buffer = ~dict.at(number);

      if((buffer.numChannels == 1), {
        Synth(\playMono, [\buf, buffer.bufnum])
      }, {
        if((buffer.numChannels == 2), {
          Synth(\playStereo, [\buf, buffer.bufnum])
        })
      });
    };
  };

  n = NetAddr.new("127.0.0.1");
  o = OSCFunc(~handler, '/sampler', n, 49162);
});