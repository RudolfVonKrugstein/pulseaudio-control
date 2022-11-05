mod controller;
mod errors;
mod sink;
mod source;

use clap::{Parser, ValueEnum};
use clap::builder::TypedValueParser;
use controller::Controller;
use serde_json::json;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Command {
    Listen,
    ListSinks,
    ListSources,
    SetSink,
    SetSource,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    command: Command,
    #[arg(short, long)]
    index: Option<String>,
    #[arg(short, long)]
    output_template: Option<String>,
}

fn main() {
    let cli = Args::parse();

    // Load the template
    let template = cli.output_template.map(
        |p| std::fs::read_to_string(&p).unwrap_or_else(|_| panic!("unable to load template file at {}", p))
    );

    // Start the controller
    let mut c = Controller::new("pulseaudio-control");
    c.wait_ready().unwrap();

    match cli.command {
        Command::Listen => {},
        Command::ListSinks => list_sinks(&mut c, template).unwrap(),
        Command::ListSources => list_sources(&mut c, template).unwrap(),
        Command::SetSink => {}
        Command::SetSource => {}
    }
    c.shutdown();
}

fn list_sinks(controller: &mut Controller, template: Option<String>) -> errors::Result<()> {
    let sinks = controller.list_sinks()?;

    let result= if let Some(t) = template {
        let mut reg = handlebars::Handlebars::new();
        let template_string = std::fs::read_to_string(t)?;

        reg.render_template(template_string.as_str(), &json!(sinks))?
    } else {
        json!(sinks).to_string()
    };
    println!("{}", result);
    Ok(())
}

fn list_sources(controller: &mut Controller, template: Option<String>) -> errors::Result<()> {
    let sources = controller.list_sources()?;

    let result= if let Some(t) = template {
        let mut reg = handlebars::Handlebars::new();
        let template_string = std::fs::read_to_string(t)?;

        reg.render_template(template_string.as_str(), &json!(sources))?
    } else {
        json!(sources).to_string()
    };
    println!("{}", result);
    Ok(())
}

fn set_sink(controller: &mut Controller, index: u32) {
    controller.
}
