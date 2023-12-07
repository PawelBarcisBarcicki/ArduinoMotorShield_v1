
# Rust Library to Manage Adafruit Motor Shield

## Overview
This Rust library is designed to interface with the Adafruit Motor Shield, providing an efficient and user-friendly way to control various motors and servos. It supports DC motors, stepper motors, and servos, offering functions to control their speed, direction, and position.

## Features
- Control up to 4 DC motors.
- Support for 2 stepper motors with various stepping styles (Single, Double, Interleave, and Microstep).
- Control 2 servos with angle setting.
- Utilizes PWM via Timers (Timer0, Timer1, Timer2) for precise control.
- Digital output management for Motor Shield operations.

## Requirements
- Arduino board or compatible microcontroller.
- Adafruit Motor Shield or a compatible shield.
- Rust programming environment set up for Arduino or your microcontroller.

## Installation
- Ensure you have the Rust compiler and the appropriate microcontroller toolchain installed.
- Add this library to your Rust project's `Cargo.toml`.

## Usage
### Initialization
First, initialize the Motor Shield with the desired layout:

```rust
use arduino_motor_shield::init_ams;
use arduino_motor_shield::MotorShield;
use arduino_motor_shield::ShieldLayout;
use arduino_motor_shield::MotorPort;

let motor_shield = init_ams!(
    ShieldLayout {
        port1: MotorPort::TwoMotors,
        port2: MotorPort::SingleStepper,
    },
    peripherals,
    pins
);
```

### Controlling Motors
To control a motor, first, access the motor instance and then use the available methods:

```rust
let motor = motor_shield.motor(1).unwrap(); // Access motor 1
motor.enable();
motor.run(MotorCommands::FORWARD);
motor.speed(255); // Set speed (0-255)
// ...
motor.disable();
```

### Controlling Steppers
For stepper motors, similar methods are available:

```rust
let stepper = motor_shield.stepper(1).unwrap(); // Access stepper 1
stepper.set_speed(60); // RPM
stepper.step(100, StepperDirection::FORWARD, StepperStyle::DOUBLE);
// ...
```

### Controlling Servos
For servos, use the `set_angle` method:

```rust
let servo = motor_shield.servo(1).unwrap(); // Access servo 1
servo.set_angle(90); // Set angle (0-180)
// ...
```

## Contributing
Contributions to this library are welcome. Please ensure to follow the coding standards and add unit tests for new features.

## License
arduino_motor_shield is licensed under either of

* Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.



## Author
[Paweł "Barciś" Barcicki/BATUSystems]
