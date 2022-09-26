#![allow(dead_code)]
#![allow(unused_variables)]


mod lakes;
fn main() {
    let huron = lakes::Lake {
        name: String::from("Huron"),
        id: 1,
        flow_to_id: 2,
        polutant: 3500,
        flowrate: 11,
    };
    let erie = lakes::Lake {
        name: String::from("Erie"),
        id: 2,
        flow_to_id: 3,
        polutant: 3500,
        flowrate: 11,
    };
    let ontario = lakes::Lake {
        name: String::from("Ontario"),
        id: 3,
        flow_to_id: 4,
        polutant: 3500,
        flowrate: 11,
    };
    
}
