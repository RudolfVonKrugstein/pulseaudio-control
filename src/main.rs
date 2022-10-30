mod controller;
mod sink;
mod source;

use controller::Controller;
use clickrs::command;

#[command(name = "pulseaudio-control")]
#[argument("config", short = "c", long = "config", default_value = "~/.config/pulseaudio-control/config.yaml", help = "Path to config file")]
#[argument("template", short = "t", long = "template", default_value = None, help = "Output template")]
#[argument("sinks", short = "o", long = "sinks")]
#[argument("sources", short = "i", long = "sources")]
fn main(
    config: String,
    tamplate: Option<String>,
    sinks: bool,
    sources: bool
) {

    let mut c = Controller::new("pulseaudio-control");
    c.wait_ready();
    let sinks= c.list_sinks();
    println!("{:?}", sinks);
    let sources = c.list_sources();
    println!("{:?}", sources);
    c.shutdown();
    println!("done");
}
