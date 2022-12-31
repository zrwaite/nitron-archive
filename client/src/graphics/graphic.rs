pub fn scale(n: i32, scale: f64) -> i32 {
	(n as f64* scale) as i32
}

pub fn scale_u(n: i32, scale: f64) -> u32 {
	(n as f64 * scale) as u32
}

// pub trait HasZIndex {
// 	fn z_index(&self) -> i32;
// }