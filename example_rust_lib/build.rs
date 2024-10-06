fn main() {
	uniffi::generate_scaffolding("src/example_lib.udl").expect("A valid UDL file");
}