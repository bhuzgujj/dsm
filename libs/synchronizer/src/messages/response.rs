use bhomz::log_err;
use crate::messages::bytify::Bytify;
use crate::messages::commands::Commands;

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
pub enum Response<I>
where
	I: Bytify,
{
	Acknowledged {
		msg_id: I,
	},
	Refusal {
		msg_id: I,
	},
}

impl<I> Response<I>
where
	I: Bytify,
{
	pub(crate) fn get_id(&self) -> u8 {
		match self {
			Response::Acknowledged { .. } => 0,
			Response::Refusal { .. } => 1
		}
	}
}

impl<I> Commands for Response<I>
where
	I: Bytify,
{
	fn serialize(&self) -> anyhow::Result<Vec<u8>> {
		let mut bytes = vec![self.get_id()];
		match self {
			Response::Acknowledged { msg_id } => bytes.append(&mut msg_id.to_sized_bytes()),
			Response::Refusal { msg_id } => bytes.append(&mut msg_id.to_sized_bytes())
		}
		Ok(bytes)
	}

	fn deserialize(bytes: &[u8], index: usize) -> anyhow::Result<(Self, usize)> {
		match &bytes[index] {
			0 => {
				let (msg_id, next ) = I::try_from_sized_bytes(&bytes, index + 1)?;
				Ok((Response::Acknowledged { msg_id }, next))
			},

			1 => {
				let (msg_id, next ) = I::try_from_sized_bytes(&bytes, index + 1)?;
				Ok((Response::Refusal { msg_id }, next))
			},

			id => log_err!("Response id {} does not exist", id)
		}
	}
}