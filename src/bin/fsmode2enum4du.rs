use std::io;
use std::process::ExitCode;

use rs_fsmode2enum4du::ModeSource;
use rs_fsmode2enum4du::PrintMode;

fn mode_source() -> impl ModeSource {
    rs_fsmode2enum4du::modesrc_txt_stdin()
}

fn mode_sink() -> impl PrintMode {
    rs_fsmode2enum4du::wtr2mode_printer_asn1_der(io::stdout())
}

fn io_main() -> impl Fn() -> Result<(), io::Error> {
    move || {
        let mut source = mode_source();
        let mut sink = mode_sink();
        source.to_sink(&mut sink)
    }
}

fn sub() -> Result<(), io::Error> {
    io_main()()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
