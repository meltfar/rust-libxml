//! The parser functionality

use c_signatures::*;
use global::*;
use tree::*;

use std::ffi::CString;
use std::fmt;

enum HtmlParserOption {
    HtmlParseRecover = 1, // Relaxed parsing
    // HTML_PARSE_NODEFDTD = 4, // do not default a doctype if not found
    HtmlParseNoerror = 32, // suppress error reports
    HtmlParseNowarning = 64, // suppress warning reports
    // HTML_PARSE_PEDANTIC = 128, // pedantic error reporting
    // HTML_PARSE_NOBLANKS = 256, // remove blank nodes
    // HTML_PARSE_NONET = 2048, // Forbid network access
    // HTML_PARSE_NOIMPLIED = 8192, // Do not add implied html/body... elements
    // HTML_PARSE_COMPACT = 65536, // compact small text nodes
    // HTML_PARSE_IGNORE_ENC = 2097152, // ignore internal document encoding hint
}

///Parser Errors
pub enum XmlParseError {
    ///Parsing returned a null pointer as document pointer
    GotNullPointer,
}

impl fmt::Debug for XmlParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            XmlParseError::GotNullPointer => write!(f, "Got a Null pointer")
        }
    }
}
#[derive(PartialEq)]
pub enum ParseFormat {
  XML,
  HTML
}
pub struct Parser {
  pub format : ParseFormat
}
impl Default for Parser {
  fn default() -> Self {
    _libxml_global_init();
    Parser { format : ParseFormat::XML}
  }
}
impl Parser {
  pub fn default_html() -> Self {
    _libxml_global_init();
    Parser { format : ParseFormat::HTML}
  }
  ///Parses the XML/HTML file `filename` to generate a new `Document`
  pub fn parse_file(&self, filename : &str) -> Result<Document, XmlParseError> {
    let c_filename = CString::new(filename).unwrap().as_ptr();
    let c_utf8 = CString::new("utf-8").unwrap().as_ptr();
    match self.format {
      ParseFormat::XML => { unsafe {
        let docptr = xmlParseFile(c_filename);
        match docptr.is_null() {
          true => Err(XmlParseError::GotNullPointer),
          false => Ok(Document::new_ptr(docptr))
        } }
      },
      ParseFormat::HTML => {
        // TODO: Allow user-specified options later on
        unsafe {
          let options : u32 = HtmlParserOption::HtmlParseRecover as u32 +
          HtmlParserOption::HtmlParseNoerror as u32 +
          HtmlParserOption::HtmlParseNowarning as u32;
          let docptr = htmlReadFile(c_filename, c_utf8, options);
          match docptr.is_null() {
            true => Err(XmlParseError::GotNullPointer),
            false => Ok(Document::new_ptr(docptr))
          }
        }
      }
    }
  }
}

impl Drop for Parser {
  fn drop(&mut self) {
    _libxml_global_drop();
  }
}