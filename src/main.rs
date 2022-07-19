use std::collections::VecDeque;
use std::process::Command;

use gilrs::{Gilrs, Button, Event, EventType};

fn handle_button(start_code: &[Button], button_queue: &mut VecDeque<Button>, pressed_button: Button) {
    // Update button queue
    println!("Pressed {:?}", pressed_button);
    button_queue.push_back(pressed_button);
    if button_queue.len() > start_code.len() {
        button_queue.pop_front();
    }

    // Check if the macro code has been entered
    for element in start_code.into_iter().enumerate() {
        let (index, button): (usize, &Button) = element;
        if button_queue.get(index) != Some(&button) {
            return;
        }
    }

    println!("Found the macro code!");
    button_queue.clear(); // Have to clear to prevent activating again too soon

    // Macro Temp
    let _ = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "C:\\Program Files (x86)\\Steam\\steam.exe", "steam://open/bigpicture"])
            .spawn()
            .expect("Failed to open Steam.");
    } else { println!("Get a PC, Huntis.") }; // I'll add other platforms / functions later
}

fn main() {
    let mut gilrs: Gilrs = Gilrs::new().unwrap();

    // This is the sequence to begin the controller macropad behavior
    let start_code = [
        Button::Start,
        Button::Select,
        Button::Start
    ];
    let mut button_queue: VecDeque<Button> = VecDeque::with_capacity(start_code.len());

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
            }

            // println!("{:?}", event);
            match event {
                EventType::ButtonReleased(Button::East, ..) => handle_button(&start_code, &mut button_queue, Button::East),
                EventType::ButtonReleased(Button::South, ..) => handle_button(&start_code, &mut button_queue, Button::South),
                EventType::ButtonReleased(Button::West, ..) => handle_button(&start_code, &mut button_queue, Button::West),
                EventType::ButtonReleased(Button::North, ..) => handle_button(&start_code, &mut button_queue, Button::North),
                EventType::ButtonReleased(Button::Start, ..) => handle_button(&start_code, &mut button_queue, Button::Start),
                EventType::ButtonReleased(Button::Select, ..) => handle_button(&start_code, &mut button_queue, Button::Select),
                _ => (),
            }
        }
    }
}
