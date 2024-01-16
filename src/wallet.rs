use super::*;

pub(crate) struct Wallet {
	_private: (),
}

impl Wallet {
	pub(crate) fn load(options: &Options) -> Result<Self> {
		
		Ok(Self { _private: () })
	}
}