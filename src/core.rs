//! Contains definitions of core pnml types independent of net type.
//!
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};
use xml_util::*;

pub type Id = String;

///Ensures proper PNML document structure and then passes control to the read_net function
pub fn read_pnml_document<T, F, R>(parser: &mut EventReader<T>, read_net: F) -> R
    where T: Read, F: Fn(&mut EventReader<T>, &XmlEvent) -> R {
    inside("pnml", parser, |p1| {
        drop_until(p1, |p2, e| {
            match e {
                &XmlEvent::StartElement { ref name, .. } if name.local_name == "net" => Some(read_net(p2, e)),
                _ => None
            }
        }).expect("No Net found in the document.")
    })
}
