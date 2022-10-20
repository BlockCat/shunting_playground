

mod location;
mod reader;

fn main() {
    let path = include_str!("../../data/Kleine_Binckhorst.location.coords.yaml");
    let locations: LocationResource = serde_yaml::from_str(path).expect("Could not read locations");

    println!("Hello, world: {:?}", locations);
}
