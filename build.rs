// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // tonic_build::compile_protos("pb/authmanager/gateway.proto")?;
//     // tonic_build::compile_protos("pb/snowflake/circuit.proto")?;
//     // tonic_build::compile_protos("pb/snowflake/job.proto")?;
//     tonic_build::configure()
//         .compile(&["pb/snowflake/"], &["pb"])
//         .unwrap();
//     Ok(())
// }
fn main() -> Result<(), Box<dyn std::error::Error>> {
   tonic_build::configure()
        .build_server(false)
        .compile(
            &["pb/pbsnow/job.proto"],
            &["pb/pbsnow"],
        )?;

   tonic_build::configure()
        .build_server(false)
        .compile(
            &["pb/authmanager/gateway.proto"],
            &["pb/authmanager"],
        )?;

   Ok(())
}
