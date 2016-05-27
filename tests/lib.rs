extern crate pnml;

use pnml::pt_net::parser::*;
use pnml::pt_net::Net;
use pnml::pt_net::Element;
use pnml::pt_net::Element::*;

#[test]
fn empty_net() {
	let from_file = read_pt_file("tests/pt_nets/empty.pnml");
	let expected = Net { id: "empty-net".to_string(), elements: vec![] };
	assert_eq!(expected, from_file);
}

#[test]
fn place_net() {
	let from_file = read_pt_file("tests/pt_nets/place.pnml");
	let expected = Net {
		id: "place-net".to_string(), 
		elements: vec![ Place {
			id: "place".to_string(),
			initial_marking: 100,
		}]
	};	
	assert_eq!(expected, from_file);
}

#[test]
fn small_net() {
	let from_file = read_pt_file("tests/pt_nets/small.pnml");
	let expected = Net {
		id: "small-net".to_string(),
		elements: vec![ Place {
			id: "place".to_string(),
			initial_marking: 100,
		}, Transition {
			id: "transition".to_string(),			
		}, Arc {
			source: "transition".to_string(),
			target: "place".to_string(),
			id: "arc1".to_string(),
			inscription: 12,
		}, Arc {
			source: "place".to_string(),
			target: "transition".to_string(),
			id: "arc2".to_string(),
			inscription: 3,
		} ],
	};
	assert_eq!(expected, from_file);
}

#[test]
fn pool() {
	let from_file = read_pt_file("tests/pt_nets/pool.pnml");
	let expected = Net {
		id: "SwimmingPool-PT-01".to_string(),

		elements: vec![ 
			Element::new_transition("GetK"),
			Element::new_place("Entered", 0),
			Element::new_transition("GetB"),
			Element::new_place("WaitBag", 0),
			Element::new_transition("RelK"),
			Element::new_place("Undress", 0),
			Element::new_transition("GetK2"),
			Element::new_place("InBath", 0),
			Element::new_transition("RBag"),
			Element::new_place("Dress", 0),
			Element::new_transition("RKey"),
			Element::new_place("Dressed", 0),
			Element::new_transition("Enter"),
			Element::new_place("Out", 20),
			Element::new_place("Cabins", 10),
			Element::new_place("Bags", 15),	
			Element::new_arc("cId901051587913946698728", "Dressed", "RKey", 1),		 
			Element::new_arc("cId901051587913946698718", "Entered", "GetK", 1),
			Element::new_arc("cId900390594197776317929", "RKey", "Out", 1),
			Element::new_arc("cId900390594197776317919", "GetK", "WaitBag", 1),	//yes, the id is an error in the model
			Element::new_arc("cId900390594197776317930", "Out", "Enter", 1),
			Element::new_arc("cId900390594197776317920", "WaitBag", "GetB", 1),
		    Element::new_arc("cId900390594197776317931", "Enter", "Entered", 1),
		    Element::new_arc("cId900390594197776317921", "GetB", "Undress", 1),
		    Element::new_arc("cId900390594197776317933", "RelK", "Cabins", 1),
		    Element::new_arc("cId900390594197776317922", "Undress", "RelK", 1),
		    Element::new_arc("cId900390594197776317934", "Cabins", "GetK2", 1),
		    Element::new_arc("cId900390594197776317923", "RelK", "InBath", 1),
		    Element::new_arc("cId900390594197776317935", "RKey", "Cabins", 1),
		    Element::new_arc("cId900390594197776317924", "InBath", "GetK2", 1),
		    Element::new_arc("cId900390594197776317936", "Bags", "GetB", 1),
		    Element::new_arc("cId900390594197776317925", "GetK2", "Dress", 1),
		    Element::new_arc("cId900390594197776317937", "RBag", "Bags", 1),
		    Element::new_arc("cId900390594197776317926", "Dress", "RBag", 1),
		    Element::new_arc("cId900390594197776317948", "Cabins", "GetK", 1),
		    Element::new_arc("cId900390594197776317927", "RBag", "Dressed", 1),
		]	   
	};
	assert_eq!(expected, from_file);
}