fn main() {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/gobgp")  // you can change the generated code's location
        .compile(
            &["proto/gobgp/api/gobgp.proto"],
            &["proto/gobgp/api"], // specify the root location to search proto dependencies
        ).unwrap();
}
