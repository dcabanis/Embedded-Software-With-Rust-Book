#![cfg_attr(not(test), no_std)]

//! The fan state machine from section 8.2.1, kept separate from the firmware
//! entry point so its transition table can be unit-tested on the host.

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FanState {
    Off,
    Low,
    Medium,
    High,
    Stalled,
}

#[derive(Clone, Copy)]
pub enum Event {
    ButtonPress,
    StallDetected,
}

pub struct Fan {
    state: FanState,
}

impl Fan {
    pub const fn new() -> Self {
        Fan { state: FanState::Off }
    }

    pub fn state(&self) -> FanState {
        self.state
    }

    /// Feed one event into the machine.
    pub fn handle(&mut self, event: Event) {
        let next_state = Self::next(self.state, event);
        self.state = next_state;
    }

    /// The transition function: no hardware, no locks, no side effects.
    fn next(current_state: FanState, event: Event) -> FanState {
        use Event::*;
        use FanState::*;
        match current_state {
            Off => match event {
                ButtonPress => Low,
                // Nothing is spinning, so nothing can stall.
                StallDetected => Off,
            },

            Low => match event {
                ButtonPress => Medium,
                StallDetected => Stalled,
            },

            Medium => match event {
                ButtonPress => High,
                StallDetected => Stalled,
            },

            High => match event {
                ButtonPress => Off,
                StallDetected => Stalled,
            },

            Stalled => match event {
                // A button press here means "reset", not "cycle forward".
                ButtonPress => Off,
                // We already know it's stalled; a second signal changes nothing.
                StallDetected => Stalled,
            },
        }
    }
}

impl Default for Fan {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_press_cycles_through_speeds() {
        let mut fan = Fan::new();
        assert_eq!(fan.state(), FanState::Off);

        fan.handle(Event::ButtonPress);
        assert_eq!(fan.state(), FanState::Low);

        fan.handle(Event::ButtonPress);
        assert_eq!(fan.state(), FanState::Medium);

        fan.handle(Event::ButtonPress);
        assert_eq!(fan.state(), FanState::High);

        fan.handle(Event::ButtonPress);
        assert_eq!(fan.state(), FanState::Off);
    }

    #[test]
    fn stall_while_off_stays_off() {
        let mut fan = Fan::new();
        fan.handle(Event::StallDetected);
        assert_eq!(fan.state(), FanState::Off);
    }

    #[test]
    fn stall_from_any_running_speed_goes_to_stalled() {
        for presses in 1..=3 {
            let mut fan = Fan::new();
            for _ in 0..presses {
                fan.handle(Event::ButtonPress);
            }
            fan.handle(Event::StallDetected);
            assert_eq!(fan.state(), FanState::Stalled);
        }
    }

    #[test]
    fn button_press_from_stalled_resets_to_off() {
        let mut fan = Fan::new();
        fan.handle(Event::ButtonPress); // Off -> Low
        fan.handle(Event::StallDetected); // Low -> Stalled
        assert_eq!(fan.state(), FanState::Stalled);

        fan.handle(Event::ButtonPress); // Stalled -> Off (reset, not cycle)
        assert_eq!(fan.state(), FanState::Off);
    }

    #[test]
    fn second_stall_while_stalled_changes_nothing() {
        let mut fan = Fan::new();
        fan.handle(Event::ButtonPress);
        fan.handle(Event::StallDetected);
        fan.handle(Event::StallDetected);
        assert_eq!(fan.state(), FanState::Stalled);
    }
}
