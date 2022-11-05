use error_chain::error_chain;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {
        UnexpectedPAMainLoopQuit
    }

    foreign_links {
        PA(::libpulse_binding::error::PAErr);
        IO(::std::io::Error);
        Template(::handlebars::TemplateError);
        Render(::handlebars::RenderError);
        // SendSinkError(::std::sync::mpsc::SendError<Option<crate::sink::Sink>>);
        // SendSourceError(::std::sync::mpsc::SendError<Option<crate::source::Source>>);
    }
}
