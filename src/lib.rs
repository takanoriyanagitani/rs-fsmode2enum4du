use std::io;

use std::fs::FileType;

use io::Read;
use io::Write;

#[repr(u8)]
#[derive(Default, Debug, Clone, Copy)]
pub enum DirentType4du {
    #[default]
    Others = 0,
    Dir = 1,
    Reg = 2,
    Sym = 3,
}

pub const fn dtyp4du2u8(dtyp: DirentType4du) -> u8 {
    dtyp as u8
}

impl From<FileType> for DirentType4du {
    fn from(ftyp: FileType) -> Self {
        if ftyp.is_dir() {
            return Self::Dir;
        }

        if ftyp.is_file() {
            return Self::Reg;
        }

        if ftyp.is_symlink() {
            return Self::Sym;
        }

        Self::Others
    }
}

#[cfg(unix)]
use std::{fs::Metadata, os::unix::fs::MetadataExt};

pub const MET2TYP4DU: [u8; 16] = [
    0b00, // other
    0b00, // other(fifo)
    0b00, // other(char)
    0b00, // other
    0b01, // dir
    0b00, // other
    0b00, // other(block)
    0b00, // other
    0b10, // regular
    0b00, // other
    0b11, // symlink
    0b00, // other
    0b00, // other(socket)
    0b00, // other
    0b00, // other
    0b00, // other
];

pub fn mode2mapd(mode: u32) -> u8 {
    let typix: u32 = (mode >> 12) & 0x0f;
    MET2TYP4DU[typix as usize]
}

pub trait PrintMode {
    fn sink(&mut self, mode: u32) -> Result<(), io::Error>;
}

impl<F> PrintMode for F
where
    F: FnMut(u32) -> Result<(), io::Error>,
{
    fn sink(&mut self, mode: u32) -> Result<(), io::Error> {
        self(mode)
    }
}

pub fn print_mode_simple(mode: u32) -> Result<(), io::Error> {
    println!("mode: {mode}");
    Ok(())
}

pub fn mode_printer_simple() -> impl PrintMode {
    print_mode_simple
}

pub fn wtr2mode_printer_asn1_der<W>(mut wtr: W) -> impl PrintMode
where
    W: Write,
{
    move |mode: u32| {
        let converted: u8 = mode2mapd(mode);
        wtr.write_all(&[0x0a])?; // enum
        wtr.write_all(&[0x01])?; // length: 1
        wtr.write_all(&[converted])?;
        Ok(())
    }
}

pub trait ModeSource {
    fn get_mode(&mut self) -> Result<u32, io::Error>;

    fn to_sink<S>(&mut self, sink: &mut S) -> Result<(), io::Error>
    where
        S: PrintMode,
    {
        let mode: u32 = self.get_mode()?;
        sink.sink(mode)
    }
}

impl<F> ModeSource for F
where
    F: FnMut() -> Result<u32, io::Error>,
{
    fn get_mode(&mut self) -> Result<u32, io::Error> {
        self()
    }
}

pub fn rdr2src_txt<R>(rdr: R) -> impl ModeSource
where
    R: Read,
{
    let mut taken = rdr.take(128);
    move || {
        let mut buf: String = String::default();
        taken.read_to_string(&mut buf)?;
        str::parse(&buf).map_err(io::Error::other)
    }
}

pub fn modesrc_txt_stdin() -> impl ModeSource {
    rdr2src_txt(io::stdin())
}

#[cfg(unix)]
impl From<&Metadata> for DirentType4du {
    fn from(m: &Metadata) -> Self {
        let mode: u32 = m.mode();
        let mapd: u8 = mode2mapd(mode);
        match mapd {
            1 => Self::Dir,
            2 => Self::Reg,
            3 => Self::Sym,
            _ => Self::Others,
        }
    }
}
