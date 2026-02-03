pub trait Commands
where
	Self: Sized
{
	fn serialize(&self) -> anyhow::Result<Vec<u8>>;
	fn deserialize(bytes: &[u8], index: usize) -> anyhow::Result<(Self, usize)>;
}