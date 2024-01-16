use super::*;

#[derive(Clone, Default, Debug, Parser)]
pub(crate) struct Options {
	#[clap(long, default_value = "bells", help = "Use wallet <WALLET>.")]
	pub(crate) wallet: String,
	#[clap(long, default_value = "http://bells.quark.blue/api/", help = "Use API URL <API>.")]
	pub(crate) api_url: String,

}

impl Options {

	// pub(crate) fn load_config(&self) -> Result<Config> {
	//   match &self.config {
	//     Some(path) => Ok(serde_yaml::from_reader(File::open(path)?)?),
	//     None => match &self.config_dir {
	//       Some(dir) if dir.join("bells.yaml").exists() => {
	//         Ok(serde_yaml::from_reader(File::open(dir.join("bells.yaml"))?)?)
	//       }
	//       Some(_) | None => Ok(Default::default()),
	//     },
	//   }
	// }

}