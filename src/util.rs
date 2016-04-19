pub fn bytes(bits: usize) -> usize {
	(bits as f32 / 8.0).ceil() as usize
}
