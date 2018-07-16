use addr2line;
use findshlibs::{self, Segment, SharedLibrary};
use gimli;
use object::{self, Object};
use std::u32;

use SymbolName;

type Dwarf = addr2line::Context<gimli::EndianRcSlice<gimli::RunTimeEndian>>;
type Symbols<'map> = object::SymbolMap<'map>;

struct Mapping {
    dwarf: Dwarf,
    // 'static lifetime is a lie to hack around lack of support for self-referential structs.
    symbols: Symbols<'static>,
    _map: Mmap,
}

impl Mapping {
    fn new(binary: &'static [u8]) -> Mapping {
        let (dwarf, symbols) = {
            let object = object::File::parse(&*map).ok()?;
            let dwarf = addr2line::Context::new(&binary).ok()?;
            let symbols = object.symbol_map();
            (dwarf, unsafe { mem::transmute(symbols) })
        };
        Mapping {
            dwarf,
            symbols,
            _map: map,
        }
    }
}

pub fn resolve(addr: *mut u8, cb: &mut FnMut(&super::Symbol)) {
    let m = Mapping::new(&[0]);

    let mut found_sym = false;

    if let Ok(mut frames) = m.dwarf.find_frames(addr as u64) {
        while let Ok(Some(frame)) = frames.next() {
            let (file, line) = frame
                .location
                .map(|l| (l.file, l.line))
                .unwrap_or((None, None));
            let name = frame
                .function
                .and_then(|f| f.raw_name().ok().map(|f| f.to_string()));
            let sym = super::Symbol {
                inner: Symbol::new(addr.0 as usize, file, line, name),
            };
            cb(&sym);
            found_sym = true;
        }
    }

    // No DWARF info found, so fallback to the symbol table.
    if !found_sym {
        if let Some(name) = m.symbols.get(addr.0 as u64).and_then(|x| x.name()) {
            let sym = super::Symbol {
                inner: Symbol::new(addr.0 as usize, None, None, Some(name.to_string())),
            };
            cb(&sym);
        }
    }
}

pub struct Symbol {
    addr: usize,
    file: Option<String>,
    line: Option<u64>,
    name: Option<String>,
}

impl Symbol {
    fn new(addr: usize, file: Option<String>, line: Option<u64>, name: Option<String>) -> Symbol {
        Symbol {
            addr,
            file,
            line,
            name,
        }
    }

    pub fn name(&self) -> Option<SymbolName> {
        self.name.as_ref().map(|s| SymbolName::new(s.as_bytes()))
    }

    pub fn addr(&self) -> Option<*mut c_void> {
        Some(self.addr as *mut c_void)
    }

    pub fn filename(&self) -> Option<&Path> {
        self.file.as_ref().map(|f| f.as_ref())
    }

    pub fn lineno(&self) -> Option<u32> {
        self.line.and_then(|l| {
            if l > (u32::MAX as u64) {
                None
            } else {
                Some(l as u32)
            }
        })
    }
}
