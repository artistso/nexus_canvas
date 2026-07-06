fn main() {
    uniffi::generate_scaffolding("./src/nexus_engine.udl")
        .expect("Failed to generate UDL scaffolding");
    println!("cargo:rerun-if-changed=src/nexus_engine.udl");
}
