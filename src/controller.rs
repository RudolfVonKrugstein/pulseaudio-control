extern crate libpulse_binding as pulse;

use std::sync::mpsc::{channel, TryRecvError, Sender, Receiver};

use std::borrow::{Borrow, Cow};
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use libpulse_binding::callbacks::ListResult;
use libpulse_binding::context::introspect::SinkInfo;
use pulse::mainloop::standard::Mainloop;
use pulse::context::{Context, FlagSet as ContextFlagSet};
use pulse::context::subscribe::Facility;
use pulse::stream::{Stream, FlagSet as StreamFlagSet};
use pulse::sample::{Spec, Format};
use pulse::proplist::Proplist;
use pulse::mainloop::standard::IterateResult;
use pulse::def::Retval;
use crate::sink::Sink;
use crate::source::Source;


pub struct Controller {
    mainloop: Rc<RefCell<Mainloop>>,
    context: Rc<RefCell<Context>>
}

impl Controller {
    pub fn new(app_name: &str) -> Controller {
        let mut proplist = Proplist::new().unwrap();
        proplist.set_str(pulse::proplist::properties::APPLICATION_NAME, "FooApp")
            .unwrap();
        let mainloop:Rc<RefCell<Mainloop>> = Rc::new(RefCell::new(Mainloop::new()
            .expect("Failed to create mainloop")));

        let context = Rc::new(RefCell::new(Context::new_with_proplist(
            mainloop.borrow_mut().deref(),
            "FooAppContext",
            &proplist
        ).expect("Failed to create new context")));

        context.borrow_mut().connect(None, ContextFlagSet::NOFLAGS, None)
            .expect("Failed to connect context");
        Controller {
            mainloop,
            context
        }
    }

    pub fn wait_ready(&mut self) {
        // Wait for context to be ready
        loop {
            match self.mainloop.borrow_mut().iterate(false) {
                IterateResult::Quit(_) |
                IterateResult::Err(_) => {
                    eprintln!("Iterate state was not success, quitting...");
                    return;
                },
                IterateResult::Success(_) => {},
            }
            match self.context.borrow_mut().get_state() {
                pulse::context::State::Ready => {
                    println!("conext is ready!");
                    break;
                },
                pulse::context::State::Failed |
                pulse::context::State::Terminated => {
                    eprintln!("Context state failed/terminated, quitting...");
                    return;
                },
                _ => {},
            }
        }
    }

     pub fn shutdown(&mut self) {
         self.context.borrow_mut().disconnect();
         self.mainloop.borrow_mut().quit(Retval(0)); // uncertain whether this is necessary
     }

    pub fn list_sinks(&mut self) -> Vec<Sink> {
        let (sender, receiver) = channel::<Option<Sink>>();
        self.context.borrow_mut().introspect().get_sink_info_list(move |s| {
            match s {
                ListResult::Item(i) => {
                    sender.send(Some(Sink::from_sink_info(i)));
                }
                ListResult::End => {
                    sender.send(None);
                }
                ListResult::Error => {}
            }
        });

        self.collect_list(&receiver)
    }

    pub fn list_sources(&mut self) -> Vec<Source> {
        let (sender, receiver) = channel::<Option<Source>>();
        self.context.borrow_mut().introspect().get_source_info_list(move |s| {
            match s {
                ListResult::Item(i) => {
                    sender.send(Some(Source::from_source_info(i)));
                }
                ListResult::End => {
                    sender.send(None);
                }
                ListResult::Error => {}
            }
        });

        self.collect_list(&receiver)
    }

    fn collect_list<T>(&self, receiver :&Receiver<Option<T>>) -> Vec<T> {
        let mut res= Vec::new();
        loop {
            match self.mainloop.borrow_mut().iterate(true) {
                IterateResult::Success(_) => {
                    loop {
                        match receiver.try_recv() {
                            Ok(v) => {
                                match v {
                                    Some(v) => res.push(v),
                                    None => {
                                        return res
                                    }
                                }
                            }
                            Err(e) => match e {
                                TryRecvError::Empty => {
                                    // nothing to do by continue
                                    break;
                                }
                                TryRecvError::Disconnected => {
                                    // Done!
                                    return res;
                                }
                            }
                        }
                    }
                }
                IterateResult::Quit(_) => {
                    panic!("unexpected quiting of main loop");
                }
                IterateResult::Err(e) => {
                    panic!("error in main loop: {}", e.to_string().unwrap());
                }
            }
        };
        res
    }
}

