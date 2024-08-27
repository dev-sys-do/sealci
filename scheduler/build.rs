use std::error::Error;
use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
	let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

	tonic_build::configure()
		.file_descriptor_set_path(out_dir.join("scheduler_descriptor.bin"))
		.compile(
			&[
				"../api/proto/scheduler/agent.proto",
				"../api/proto/scheduler/controller.proto",
				"../api/proto/agent/actions.proto",
			],
			&["../api/proto"])?;

	Ok(())
}
