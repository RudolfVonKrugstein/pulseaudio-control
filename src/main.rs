extern crate libpulse_binding as pulse;

use std::sync::atomic;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use pulse::mainloop::standard::Mainloop;
use pulse::context::{Context, FlagSet as ContextFlagSet};
use pulse::stream::{Stream, FlagSet as StreamFlagSet};
use pulse::sample::{Spec, Format};
use pulse::proplist::Proplist;
use pulse::mainloop::standard::IterateResult;
use pulse::def::Retval;

fn main() {
    let spec = Spec {
        format: Format::S16NE,
        channels: 2,
        rate: 44100,
    };
    assert!(spec.is_valid());

    let mut proplist = Proplist::new().unwrap();
    proplist.set_str(pulse::proplist::properties::APPLICATION_NAME, "FooApp")
        .unwrap();

    let mut mainloop = Rc::new(RefCell::new(Mainloop::new()
        .expect("Failed to create mainloop")));

    let mut context = Rc::new(RefCell::new(Context::new_with_proplist(
        mainloop.borrow().deref(),
        "FooAppContext",
        &proplist
        ).expect("Failed to create new context")));

    context.borrow_mut().connect(None, ContextFlagSet::NOFLAGS, None)
        .expect("Failed to connect context");

    // Wait for context to be ready
    loop {
        match mainloop.borrow_mut().iterate(false) {
            IterateResult::Quit(_) |
            IterateResult::Err(_) => {
                eprintln!("Iterate state was not success, quitting...");
                return;
            },
            IterateResult::Success(_) => {},
        }
        match context.borrow().get_state() {
            pulse::context::State::Ready => { break; },
            pulse::context::State::Failed |
            pulse::context::State::Terminated => {
                eprintln!("Context state failed/terminated, quitting...");
                return;
            },
            _ => {},
        }
    }

    context.borrow_mut().set_subscribe_callback(Some(Box::new(|a,b,c| {})));
    context.borrow_mut().subscribe(pulse::context::subscribe::InterestMaskSet::all(), |done| {});

    let mut stream = Rc::new(RefCell::new(Stream::new(
        &mut context.borrow_mut(),
        "Music",
        &spec,
        None
        ).expect("Failed to create new stream")));

    stream.borrow_mut().connect_playback(None, None, StreamFlagSet::START_CORKED,
        None, None).expect("Failed to connect playback");

    // Wait for stream to be ready
    loop {
        match mainloop.borrow_mut().iterate(false) {
            IterateResult::Quit(_) |
            IterateResult::Err(_) => {
                eprintln!("Iterate state was not success, quitting...");
                return;
            },
            IterateResult::Success(_) => {},
        }
        match stream.borrow().get_state() {
            pulse::stream::State::Ready => { break; },
            pulse::stream::State::Failed |
            pulse::stream::State::Terminated => {
                eprintln!("Stream state failed/terminated, quitting...");
                return;
            },
            _ => {},
        }
    }

    // Our main logic (to output a stream of audio data)
    let drained = Rc::new(RefCell::new(false));
    loop {
        match mainloop.borrow_mut().iterate(false) {
            IterateResult::Quit(_) |
            IterateResult::Err(_) => {
                eprintln!("Iterate state was not success, quitting...");
                return;
            },
            IterateResult::Success(_) => {},
        }

        // Write some data with stream.write()

        if stream.borrow().is_corked().unwrap() {
            stream.borrow_mut().uncork(None);
        }

        // Wait for our data to be played
        let _o = {
            let drain_state_ref = Rc::clone(&drained);
            stream.borrow_mut().drain(Some(Box::new(move |_success: bool| {
                *drain_state_ref.borrow_mut() = true;
            })))
        };
        while *drained.borrow_mut() == false {
            match mainloop.borrow_mut().iterate(false) {
                IterateResult::Quit(_) |
                IterateResult::Err(_) => {
                    eprintln!("Iterate state was not success, quitting...");
                    return;
                },
                IterateResult::Success(_) => {},
            }
        }
        *drained.borrow_mut() = false;

        // Remember to break out of the loop once done writing all data (or whatever).
    }

    // Clean shutdown
    mainloop.borrow_mut().quit(Retval(0)); // uncertain whether this is necessary
    stream.borrow_mut().disconnect().unwrap();
}
