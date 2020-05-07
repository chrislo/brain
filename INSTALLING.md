# Installing on a Raspberry Pi 4

I've got a Samsung EVO Select 32GB microSDHC card. Etched with Raspberry Pi Buster Lite image using belenaEtcher.

I added `ssh` to the `boot` volume to enable SSH on boot. To connect to the WiFi on boot I also added a `wpa_supplicant.conf` file with the following:

    ctrl_interface=DIR=/var/run/wpa_supplicant GROUP=netdev
    update_config=1
    country=GB

    network={
     ssid="<ssid>"
     psk="<password>"
    }

Powered up the pi with the SD card in, and after boot could ssh into `raspberrypi.local`.

# Installing OSMID (o2m / m2o)

    sudo apt-get update
    sudo apt-get install git libasound2-dev libx11-dev

    mkdir src
    cd src
    git clone https://github.com/llloret/osmid.git

    cd osmid
    mkdir build
    cd build
    cmake ..
    make

    sudo cp o2m /usr/local/bin/
    sudo cp m2o /usr/local/bin/

# Installing Jack

<https://madskjeldgaard.dk/posts/raspi4-notes/>

    sudo apt-get install jackd2

I said "yes" when asked if I wanted to enable realtime priority.

Created a `~/.jackdrc` file with the following:

    /usr/bin/jackd -P75 -dalsa -dhw:0 -r44100 -p512 -n3

Where `hw:0` referred to the name of the internal soundcard reported by `aplay -l`.

# Some tweaks to Raspbian to improve audio performance

<https://madskjeldgaard.dk/posts/raspi4-notes/>

    sudo apt-get install cpufrequtils
    sudo cpufreq-set -r -g performance

    sudo echo "ENABLE="true"
    GOVERNOR="performance"
    MAX_SPEED="0"
    MIN_SPEED="0" " | sudo tee -a /etc/default/cpufrequtils

    # Set realtime priority and memlock
    sudo echo "
    @audio nice -15
    @audio - rtprio 90       # maximum realtime priority
    @audio - memlock unlimited  # maximum locked-in-memory address space (KB)
    " | sudo tee -a /etc/security/limits.conf

    # Set swappiness
    # This setting changes the so-called swappiness of your system,
    # or in other words, the moment when your system starts to use its swap
    # partition.
    sudo echo "
    vm.swappiness = 10
    fs.inotify.max_user_watches = 524288
    " | sudo tee /etc/sysctl.conf

And then reboot

    sudo reboot

# Installing Supercollider

The debian buster apt package for supercollider is designed to run with the GUI and won't seem to run `sclang` without a display attached.

In <https://madskjeldgaard.dk/posts/raspi4-notes/> the author suggests compiling supercollider from source with the compiler flags for a GUI-less build. Here's what I did:

    sudo apt-get install libsamplerate0-dev libsndfile1-dev libasound2-dev libavahi-client-dev libreadline-dev libfftw3-dev libudev-dev cmake git libjack-jackd2-dev

    cd ~/src
    git clone --recursive --branch 3.11 https://github.com/supercollider/supercollider.git

    cd supercollider
    mkdir build
    cd build

    cmake -DCMAKE_BUILD_TYPE=Release -DSUPERNOVA=OFF -DSC_ED=OFF -DSC_EL=OFF -DSC_VIM=OFF -DNATIVE=ON -DSC_IDE=OFF -DNO_X11=ON -DSC_QT=OFF ..
    make -j4
    sudo make install
    sudo ldconfig

I haven't installed the SC plugins yet, as I'm not sure if I'll need them.

# Install rust

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    source $HOME/.cargo/env

# Install brain

    cd ~/src
    git clone https://github.com/chrislo/brain.git

    cd brain/sequencer
    cargo build --release

# Optional: Replace Pi's built in audio with external sound card

The internal sound card in the Pi isn't great. There's lots of options for external sound cards, and you can find decent 2-in and 2-out cards for not a lot of money. I bought a [Sabrent AU-MMSA](https://www.sabrent.com/product/AU-MMSA/usb-external-stereo-3d-sound-adapter-black) for around £6.

To avoid confusion, you can first disable the built-in audio interface. Edit `/boot/config.txt` and comment out the line

    #dtparam=audio=on

The reboot with `sudo reboot`.

Plug in the external USB sound card and take a note of its number within ALSA with `aplay -l`

For example, for my Sabrent card it reads

    card 1: Device [USB Audio Device], device 0: USB Audio [USB Audio]
    Subdevices: 1/1
    Subdevice #0: subdevice #0

Then edit the `~/.jackdrc` file to refer to the number of this device, in this case it's `card 1` so `hw:1`:

    /usr/bin/jackd -P75 -dalsa -dhw:1 -r44100 -p512 -n3
