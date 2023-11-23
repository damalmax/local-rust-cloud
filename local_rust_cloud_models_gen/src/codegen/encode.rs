use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter, Write};

// currently there's actually no way that encoding can fail but give it time :-)
#[non_exhaustive]
#[derive(Debug)]
pub struct XmlEncodeError {}

impl Display for XmlEncodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "error encoding XML")
    }
}

impl StdError for XmlEncodeError {}

pub struct CodeWriter<'a> {
    doc: &'a mut String,
}

impl<'a> CodeWriter<'a> {
    pub fn new(doc: &'a mut String) -> Self {
        Self { doc }
    }
}

impl<'a> CodeWriter<'a> {
    pub fn start_el<'b, 'c>(&'c mut self, tag: &'b str) -> ElWriter<'c, 'b> {
        write!(self.doc, "<{}", tag).unwrap();
        ElWriter::new(self.doc, tag)
    }
}

pub struct ElWriter<'a, 'b> {
    start: &'b str,
    doc: Option<&'a mut String>,
}

impl<'a, 'b> ElWriter<'a, 'b> {
    fn new(doc: &'a mut String, start: &'b str) -> ElWriter<'a, 'b> {
        ElWriter {
            start,
            doc: Some(doc),
        }
    }

    pub fn write_attribute(&mut self, key: &str, value: &str) -> &mut Self {
        write!(self.doc(), " {}=\"{}\"", key, value).unwrap();
        self
    }

    pub fn write_ns(mut self, namespace: &str, prefix: Option<&str>) -> Self {
        match prefix {
            Some(prefix) => {
                write!(self.doc(), " xmlns:{}=\"{}\"", prefix, namespace).unwrap()
            }
            None => write!(self.doc(), " xmlns=\"{}\"", namespace).unwrap(),
        }
        self
    }

    fn write_end(doc: &mut String) {
        write!(doc, ">").unwrap();
    }

    fn doc<'c>(&'c mut self) -> &'c mut String
    where
        'a: 'c,
    {
        // The self.doc is an Option in order to signal whether the closing '>' has been emitted
        // already (None) or not (Some). It ensures the following invariants:
        // - If finish() has been called, then self.doc is None and therefore no more writes
        //   to the &mut String are possible.
        // - When drop() is called, if self.doc is Some, then finish() has not (and will not)
        //   be called, and therefore drop() should close the tag represented by this struct.
        //
        // Since this function calls unwrap(), it must not be called from finish() or drop().
        // As finish() consumes self, calls to this method from any other method will not encounter
        // a None value in self.doc.
        self.doc.as_mut().unwrap()
    }

    pub fn finish(mut self) -> ScopeWriter<'a, 'b> {
        let doc = self.doc.take().unwrap();
        Self::write_end(doc);
        ScopeWriter {
            doc,
            start: self.start,
        }
    }
}

impl Drop for ElWriter<'_, '_> {
    fn drop(&mut self) {
        if let Some(doc) = self.doc.take() {
            // Calls to write_end() are always preceded by self.doc.take(). The value in self.doc
            // is set to Some initially, and is never reset to Some after being taken. Since this
            // transition to None happens only once, we will never double-close the XML element.
            Self::write_end(doc);
        }
    }
}

/// Wrap the construction of a tag pair `<a></a>`
pub struct ScopeWriter<'a, 'b> {
    doc: &'a mut String,
    start: &'b str,
}

impl Drop for ScopeWriter<'_, '_> {
    fn drop(&mut self) {
        write!(self.doc, "</{}>", self.start).unwrap();
    }
}

impl ScopeWriter<'_, '_> {
    pub fn data(&mut self, data: &str) {
        self.doc.write_str(data.as_ref()).unwrap();
    }

    pub fn finish(self) {
        // drop will be called which writes the closer to the document
    }

    pub fn start_el<'b, 'c>(&'c mut self, tag: &'b str) -> ElWriter<'c, 'b> {
        write!(self.doc, "<{}", tag).unwrap();
        ElWriter::new(self.doc, tag)
    }
}