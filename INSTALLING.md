# Installing on a Raspberry Pi 4

I've got a Samsung EVO Select 32GB microSDHC card. I etched it with a [Patchbox
OS](https://blokas.io/patchbox-os/) image using belenaEtcher. I chose Patchbox
OS because it comes pre-configured with `jack` and other settings for
low-latency audio.

I added `ssh` to the `boot` volume to enable SSH on boot. To connect to the WiFi
on boot I also added a `wpa_supplicant.conf` file with the following:

    ctrl_interface=DIR=/var/run/wpa_supplicant GROUP=netdev
    update_config=1
    country=GB

    network={
     ssid="<ssid>"
     psk="<password>"
    }

Powered up the pi with the SD card in, and after boot could ssh in using
`patch@<IP_ADDRESS>` using the password `blokaslabs`. I found the IP address
using `sudo nmap -sc 192.168.0.0/24` from my mac.

I then followed the [setup
wizard](https://blokas.io/patchbox-os/docs/setup-wizard/) taking note of the
recommended Jack settings.

# Optional: Replace Pi's built in audio with external sound card

The internal sound card in the Pi isn't great. There's lots of options for
external sound cards, and you can find decent 2-in and 2-out cards for not a lot
of money. I bought a [Sabrent
AU-MMSA](https://www.sabrent.com/product/AU-MMSA/usb-external-stereo-3d-sound-adapter-black)
for around £6.

To avoid confusion, you can first disable the built-in audio interface. Edit
`/boot/config.txt` and comment out the line

    #dtparam=audio=on

The reboot with `sudo reboot`.

Plug in the external USB sound card and take a note of its name within ALSA with
`aplay -l`

For example, for my Sabrent card it reads

    card 1: Device [USB Audio Device], device 0: USB Audio [USB Audio]
    Subdevices: 1/1
    Subdevice #0: subdevice #0

Run `patchbox` from the command line to restart the setup wizard and choose this
card as the default under the `jack` menu. For my Sabrent USB soundcard, I used
`-r 44100`, `-p 512` and `-n 2`.

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

# Installing Supercollider

The package for supercollider that comes with Patchbox OS is designed to run
with the GUI and as far as I can tell, won't run `sclang` headless (without a
display attached).

In <https://madskjeldgaard.dk/posts/raspi4-notes/> the author suggests compiling
supercollider from source with the compiler flags for a GUI-less build. Here's
what I did:

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

