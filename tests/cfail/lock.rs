#![deny(warnings)]
#![feature(const_fn)]
#![feature(proc_macro)]

#[macro_use(task)]
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f103xx;

use rtfm::{app, Threshold};

app! {
    device: stm32f103xx,

    resources: {
        STATE: bool = false;
        MAX: u8 = 0;
    },

    tasks: {
        EXTI0: {
            enabled: true,
            priority: 1,
            resources: [MAX, STATE],
        },

        EXTI1: {
            enabled: true,
            priority: 2,
            resources: [STATE],
        },

        EXTI2: {
            enabled: true,
            priority: 16,
            resources: [MAX],
        },
    },
}

fn init(_p: init::Peripherals, _r: init::Resources) {}

fn idle() -> ! {
    loop {}
}

task!(EXTI0, exti0);

fn exti0(mut t: Threshold, r: EXTI0::Resources) {
    // OK need to lock to access the resource
    if r.STATE.claim(&mut t, |state, _| **state) {}

    // OK can claim a resource with maximum ceiling
    r.MAX.claim_mut(&mut t, |max, _| **max += 1);
}

task!(EXTI1, exti1);

fn exti1(mut t: Threshold, r: EXTI1::Resources) {
    // ERROR no need to lock. Has direct access because priority == ceiling
    if r.STATE.claim(&mut t, |state, _| **state) {
        //~^ error no method named `claim` found for type
    }

    if **r.STATE {
        // OK
    }
}