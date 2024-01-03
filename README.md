# Project Title: Common Cathode RGB LED with RGB color rotation.

   🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀
   🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀
   🚀🚀🚀🚀🚀🚀🚀
   🚀🚀🚀

## Description:

   This project demonstrates how to control an RGB LED with a common cathode using the micro:bit V2 and the Kitronik Breakout Board Edge Connector. The code uses a timer to rotate through the colors Red, Green, and Blue, changing colors every 250 milliseconds.


## Project's hardware:

- 1 micro:bit V2
- 1 Kitronik Edge Connector Breakout Board
- 1 830 point solderless breadboard
- 1 560R resistor
- 6 M/F 6" Premium Jumper Wires (2 Red, 2 Green, 2 Blue)
- 1 F/F 6" Premium Jumper Wires (1 Black)
- 1 RGB Diffused Common Cathode LED - 5 mm
  - Forward Voltage (RGB): (2.0, 3.2, 3.2)V
  - Max forward Current (RGB): (20, 20, 20)mA
  - Max Luminosity (RGB): (2800, 6500, 1200)mcd

## Instructions:

1. Clone this repository and change/move into the cloned directory in your terminal window

2. Prepare the micro:bit V2 and Kitronik Edge Connector Breakout Board:
Attach the Kitronik Edge Connector Breakout Board to the micro:bit V2. Make sure the connectors are facing each other and that the pins are aligned.

3. Connect 3 of the 560R resistors to the breadboard so that the resistors can be shared by the RGB LED.

   Insert one end of each resistor into the following holes on the breadboard:

- d27
- d28
- d29

   Connect the 3 other ends of the 560R resistors to the breadboard in respective order to match the first insertion instructions above.

   These must be placed across the gap of the breadboard (the other side).

- f27
- f28
- f29

4. Connect the female end of the jumper wire to the anode terminals (Red, Green, Blue) of the LED using Red, Green, and Blue M/F 6" Jumper Wires. These are the three shorter ends (anode terminals) of the RGB LED legs for common cathode RGB LEDS. The word common indicates that this will be the connection to the ground pin (0V). Manufacturers also make commmon anode LEDS, but that's not what were using here for this example. 

   The anodes are the positive (+) terminals of the component. For an RGB LED, the anode terminals the longer legs and are connected to the positive side of the power source. You might be wondering, how do I test which anode lights up which color of the RGB Led - Red, Green, or Blue. We take care of that in the final step, step 13. For now, just follow the instructions as they are written.

5. You'll now have 3 M/F Jumper Wires now with the male ends availalbe. Connect the male end to the breadboard on same hole in the same line just AFTER each resistor for each anode (Red, Green, Blue), respectively.

   This is how you match up the holes from the second part of step 2 (the other end of the resistor placed across the gap) to the male ends of the Jumper Wires.

- f27 (other end of resistor) -> j27 (the male end of the Red Jumper Wire that connects to the Red anode terminal)

- f28 (other end of resistor) -> j28 (the male end of the Green Jumper Wire that connects to the Green anode terminal)

- f29 (other end of resistor) -> j29 (the male end of the Blue Jumper Wire that connects to the Blue anode terminal)

6. You'll now get 3 M/F Jumper wires. Using those, connect the male end to the breadboard on the hole in the same line just BEFORE each resistor for each Anode (Red, Green, Blue), respectively.

- a27
- a28
- a29

7. You'll now have 3 Female ends to insert into the Kitronik Edge Connector Breakout Board. 

8. Connect the Female end of the Jumper connected to the Red LED cable to the Kitronik Edge Connector Breakout Board, onto the pin set in the code.

   For example, in the code:

```rust
  let mut red_led = gpio.p0_09.into_push_pull_output(Level::Low);
```

   Would indicate that the red_led variable has been assigned to Pin 9 on the Kitronik Edge Connector Breakout Board.

9. Connect the Female end of the Jumper connected to the Green LED cable to the Kitronik Edge Connector Breakout Board, onto the pin set in the code.

10. Connect the Female end of the Jumper connected to the Blue LED cable to the Kitronik Edge Connector Breakout Board, onto the pin set in the code.

11. Connect a black F/F Jumper Wire, one female end, to the common cathode pin of the RGB LED (the shorter leg terminal). 

12. Insert the other female end of the Jumper wire into the Kitronik Breakout Board Edge Connector, to one of the 0V pins (Ground pin).

13. Build and Flash your micro:bit V2:
Connect the micro:bit V2 to your computer using the USB cable. 

   Run this command from the root cloned directory:

```rust
  - cargo build --features v2 --target thumbv7em-none-eabihf
```

   If you have problems with this, check out the Rust Discovery Book:
   [link text] https://docs.rust-embedded.org/discovery/microbit/index.html

   Then run this command from the root cloned directory:

```rust
  - cargo embed --features v2 --target thumbv7em-none-eabihf
```

   If you have problems with this, check out the Rust Discovery Book:
   [link text] https://docs.rust-embedded.org/discovery/microbit/index.html

14. The red LED, green LED, and blue LED should all light up respectively with 250 millsecond rotations. You can now test which anode terminal of your RGB LED is red, green, and blue by pulling out 2 of the 3 female Jumper Wires for the anode pins (the longer legs). You can then reconnect the cables in a color coded fashion.