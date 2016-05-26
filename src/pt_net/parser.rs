//! Defines functions for parsing pnml files into PT Nets
//!

use std::io::Read;
use xml::reader::{EventReader, XmlEvent};
use xml::reader::XmlEvent::*;
use xml::attribute::OwnedAttribute;
use xml_util::*;
use pt_net::*;
use pt_net::Element::*;
use core::*;

///Read a PT Net from a XML parser
pub fn read_pt_net<T: Read>(parser: &mut EventReader<T>) -> Net {
    read_pnml_document(parser, read_net)
}

///Read a PT Net from a PNML file
pub fn read_pt_file(file_name: &String) -> Net {
    parse_file(file_name, read_pt_net)
}

///Read net type and id
fn read_net<T: Read>(parser: &mut EventReader<T>, event: &XmlEvent) -> Net {
    match event {
        &StartElement { ref name, ref attributes, .. } if name.local_name == "net" => {
            let net_type = find_attribute("type", attributes).expect("The Net has no type.");
            if net_type != "http://www.pnml.org/version-2009/grammar/ptnet" {
                panic!("Unsupported Net type.")
            }
            Net {
                id: find_attribute("id", attributes).expect("The Net has no id."),
                elements: read_elements(parser)
            }
        }
        _ => panic!("Unexpected event {:?}", event)
    }
}

///Read the list of elements inside a net tag
fn read_elements<T: Read>(parser: &mut EventReader<T>) -> Vec<Element>  {
    collect_until("net", parser, |p, event| {
        match event {
            &StartElement { ref name, ref attributes, .. } if name.local_name == "place" => {
                Some(read_place(attributes, p))
            },
            &StartElement { ref name, ref attributes, .. } if name.local_name == "transition" => {
                Some(read_transition(attributes))
            },
            &StartElement { ref name, ref attributes, .. } if name.local_name == "arc" => {
                Some(read_arc(attributes, p))
            }
            _ => None
        }
    })
}

fn read_place<T: Read>(attributes: &Vec<OwnedAttribute>, parser: &mut EventReader<T>) -> Element {
    Place {
        id: find_attribute("id", attributes)
                .expect("Found place with no ID."),
        initial_marking: find_until("initialMarking", "place", parser, next_text)
                .unwrap_or("0".to_string()).parse()
                .expect("Error parsing initial marking: not a number.")
    }
}

fn read_transition(attributes: &Vec<OwnedAttribute>) -> Element {
    Transition {
        id: find_attribute("id", attributes)
                .expect("Found transition with no ID.")
    }
}

fn read_arc<T: Read>(attributes: &Vec<OwnedAttribute>, parser: &mut EventReader<T>) -> Element {
    Arc {
        id: find_attribute("id", attributes)
                    .expect("Found arc with no ID."),
        source: find_attribute("source", attributes)
                    .expect("Found arc with no source ID."),
        target: find_attribute("starget", attributes)
                    .expect("Found arc with no target ID."),
        inscription: find_until("inscription", "arc", parser, next_text)
                    .unwrap_or("1".to_string()).parse()
                    .expect("Error parsing arc inscription: not a number."),
    }
}
