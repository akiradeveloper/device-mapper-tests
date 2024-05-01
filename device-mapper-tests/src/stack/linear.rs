use crate::{DMStack, DMStackDecorator, DMTable, Sector};

pub struct Table {
    pub backing_dev: String,
    pub offset: Sector,
    pub len: Sector,
}
impl DMTable for Table {
    fn line(&self) -> String {
        format!(
            "0 {} linear {} {}",
            self.len.sectors(),
            self.backing_dev,
            self.offset.sectors()
        )
    }
}
pub struct Linear {
    delegate: Box<dyn DMStack>,
    table: Table,
}
impl DMStackDecorator for Linear {
    fn delegate(&self) -> &dyn DMStack {
        self.delegate.as_ref()
    }
}
impl Linear {
    pub fn new<S: DMStack + 'static>(s: S, table: Table) -> Self {
        let s = crate::reload(s, &table);
        Self {
            delegate: Box::new(s),
            table,
        }
    }
    pub fn start(&self) -> Sector {
        self.table.offset
    }
    pub fn len(&self) -> Sector {
        self.table.len
    }
}
