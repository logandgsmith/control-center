use std::collections::VecDeque;
use std::process::Command;

use gilrs::{Gilrs, Button, Event, EventType};

// struct Combo {
//     buttons: Vec<Button>
// }

struct State {
    start_code: Vec<Button>,
    button_queue: VecDeque<Button>,
    // combos: Vec<Combo>
}

impl State {
    fn update_queue(&mut self, pressed_button: Button) {
        // Update button queue
        println!("Pressed {:?}", pressed_button);
        self.button_queue.push_back(pressed_button);
        if self.button_queue.len() > self.start_code.len() {
            self.button_queue.pop_front();
        }
        println!("Current queue: {:?}", self.button_queue);
    }

    fn handle_button(&mut self, pressed_button: Button) {
        self.update_queue(pressed_button);

        // Check if the macro code has been entered
        let is_macro_entered = 
            self.start_code.len() == self.button_queue.len() &&
            self.start_code 
                .iter()
                .zip(self.button_queue.iter())
                .all(|(code, entry)| code == entry);
        
        println!("Is macro code entered: {}", is_macro_entered);

        // Stop executing if no macro detected
        if !is_macro_entered { return; }

        println!("Found the macro code!");
        self.button_queue.clear(); // Have to clear to prevent activating again too soon

        // Testing a command to run
        let _ = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "C:\\Program Files (x86)\\Steam\\steam.exe", "steam://open/bigpicture"])
                .spawn()
                .expect("Failed to open Steam.");
        } else { println!("Get a PC, Huntis.") }; // I'll add other platforms / functions later
    }
}

fn main() {
    let mut gilrs: Gilrs = Gilrs::new().unwrap();

    // This is the sequence to begin the controller macropad behavior
    let start_code = vec!( 
        Button::Start,
        Button::Select,
        Button::Start
    );

    let button_queue: VecDeque<Button> = 
        VecDeque::with_capacity(
            start_code.len()
        );

    let mut state: State = State { 
        start_code,
        button_queue 
    };

    // List connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    // Only read from one gamepad at a time
    let mut active_gamepad = None;
    
    loop {
        while let Some(Event { id, event, time: _ }) = gilrs.next_event() {
            // Set the gamepad if none is active
            if active_gamepad.is_none() {
                active_gamepad = Some(id);
                println!("{}", id);
            }

            if let EventType::ButtonPressed(button, ..) = event {
                state.handle_button(button);
            }
        }
    }
}
