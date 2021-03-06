use std::ops::Deref;
use std::rc::{Rc, Weak};
use weak_table::WeakHashSet;

#[derive(Clone, Debug, Hash)]
pub struct Symbol(Rc<str>);

impl Symbol {
    // Differentiates between variables and normal statements
    #[inline]
    pub fn is_var(&self) -> bool {
        &self.0[..1] == "?"
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        self.0.as_ptr() == other.0.as_ptr()
    }
}

impl Eq for Symbol {}

impl Deref for Symbol {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Default)]
pub struct SymbolTable(WeakHashSet<Weak<str>>);

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }

    // Returns a reference counted pointer for a given string slice
    // This function assumes the string slice is a properly formatted argument
    pub fn intern(&mut self, name: &str) -> Symbol {
        if let Some(rc) = self.0.get(name) {
            Symbol(rc)
        } else {
            let rc = Rc::<str>::from(name);
            self.0.insert(Rc::clone(&rc));
            Symbol(rc)
        }
    }
}

#[test]
fn interning() {
    let mut tab = SymbolTable::new();

    let a0 = tab.intern("a");
    let a1 = tab.intern("a");
    let b = tab.intern("b");

    assert_eq!(a0, a1);
    assert_ne!(a0, b);
}

#[test]
fn variable() {
    let mut tab = SymbolTable::new();

    let a = tab.intern("ab");
    let b = tab.intern("?a");

    assert_eq!(a.is_var(), false);
    assert_eq!(b.is_var(), true)
}
