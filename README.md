# PorcupineZero
Porcupine Hotword detection for the Raspberry Pi Zero
This is a fork of https://github.com/Picovoice/Porcupine stripped down for the Pi Zero.
The main program is implemented in rust.


Clone the Repository to your Raspberry Pi Zero

If you just want to test the hotword you can execute
    ./PorcupineZero 

To build from source install rustup:

    curl https://sh.rustup.rs -sSf | sh
    
Build it with

    cargo build
    
This will take a long time (50 minutes)


To run the tiny model: 

     ./PorcupineZero --keyword-file-path=resources/alexa_raspberrypi_tiny.ppn  --model-file-path=model/porcupine_tiny_params.pv 


CPU Requirement:

* Normal Model: 55%
* Tiny Model: 15%
