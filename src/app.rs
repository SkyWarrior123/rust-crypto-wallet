use std::str::FromStr;

#[derive(PartialEq)]
pub enum Blockchains {
	Bitcoin,
	Ethereum
}

impl FromStr for Blockchains {
	type Err = {};
	
	fn from_str(input: &str) -> Result<Blockchains, Self::Err> {
		match input {
			"bitcoin" => Ok(Blockchains::Bitcoin),
			"ethereum" => Ok(Blockchains::Ethereum),
			_ => Err(()),
		}
	}
}

impl ToString for Blockchains {
	fn to_string(&self) -> String {
		match self {
			Network::Bitcoin => return String::from("Bitcoin"),
			Network::Ethereum => return String::from("Ethereum")
		};
	}
}
