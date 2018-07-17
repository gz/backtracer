use core::u32;
use core::u64;
use elfloader;
use SymbolName;

pub fn resolve(binary: &'static [u8], addr: *mut u8, cb: &mut FnMut(&super::Symbol)) {
    let m = elfloader::ElfBinary::new("kernel", binary).expect("Can't parse");
    let mut dist = u64::MAX;
    let mut cursymbol: Option<&elfloader::elf::Symbol> = None;
    m.for_each_symbol(|esym| {
        let addr_val = addr as u64;
        if addr_val > esym.value && addr_val - esym.value < dist {
            dist = addr_val - esym.value;
            cursymbol = Some(esym);
        }
    });

    cursymbol.map(|esym| {
        let sym = super::Symbol {
            inner: Symbol::new(esym.value as usize, None, None, Some(m.symbol_name(esym))),
        };
        return cb(&sym);
    });
}

#[derive(Debug)]
pub struct Symbol {
    addr: usize,
    file: Option<&'static str>,
    line: Option<u64>,
    name: Option<&'static str>,
}

impl Symbol {
    fn new(
        addr: usize,
        file: Option<&'static str>,
        line: Option<u64>,
        name: Option<&'static str>,
    ) -> Symbol {
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

    pub fn addr(&self) -> Option<*mut u8> {
        Some(self.addr as *mut u8)
    }

    pub fn filename(&self) -> Option<&str> {
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
