mod motor_shield;

pub use crate::motor_shield::MotorShield;
pub use crate::motor_shield::layout::{ShieldLayout, MotorPort};
pub use crate::motor_shield::motors::MotorCommands;
pub use crate::motor_shield::steppers::{StepperDirection, StepperStyle};

// use arduino_hal::{
//     hal::port,
//     pac::{TC0, TC1, TC2},
//     port::{mode, Pin},
//     simple_pwm::{IntoPwmPin, Prescaler, Timer0Pwm, Timer1Pwm, Timer2Pwm}, delay_ms,
// };

// use crate::motor_shield;
// pins.d0  // no pwm
// pins.d1  // no pwm
// pins.d2  // no pwm
// pins.d3  // timer2 !!!! AF_Stepper1? AF_Motor2?
// pins.d4  // no pwm MOTORCLK
// pins.d5  // timer0 !!!! AF_Stepper2? AF_Motor4?
// pins.d6  // timer0 !!!! AF_Stepper2? AF_Motor3?
// pins.d7  // no pwm MOTORENABLE
// pins.d8  // no pwm MOTORDATA
// pins.d9  // timer1 !!!! servo2 -> >=Prescaler::Prescale256
// pins.d10 // timer1 !!!! servo1 -> >=Prescaler::Prescale256
// pins.d11 // timer2 !!!! AF_Stepper1? AF_Motor1?
// pins.d12 // no pwm MOTORLATCH
// pins.d13 // no pwm
#[macro_export]
macro_rules! init_ams {
    ($layout:expr, $p:expr, $pins:expr) => {
        MotorShield::new(
            $layout,
            $p.TC0, $p.TC1, $p.TC2, 
            $pins.d3, $pins.d4, $pins.d5, $pins.d6, $pins.d7, $pins.d8, $pins.d9, $pins.d10, $pins.d11, $pins.d12,
        )
    };
}
