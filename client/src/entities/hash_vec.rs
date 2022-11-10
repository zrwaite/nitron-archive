use std::collections::HashMap;
use std::cmp::Ordering;
use specs::DenseVecStorage;
use specs::Component;
use specs_derive::Component;

use crate::entities::GameEntity;

pub trait HasId {
	fn id(&self) -> String;
}

#[derive(Component)]
pub struct HashVec {
	vec: Vec<GameEntity>,
	cache: HashMap<String, usize>,
}
impl HashVec {
	pub fn new(mut vec: Vec<GameEntity>) -> Self {
		vec.sort_by(|a,b| a.id().cmp(&b.id()));
		Self {
			vec,
			cache: HashMap::new(),
		}
	}
	fn get_index(&mut self, id:String) -> Option<usize> {
		let index = self.cache.get(&id);
		if index.is_some() {
			return Some(*index.unwrap());
		}
		let mut low = 0;
		let mut high = self.vec.len() - 1;
		while low <= high {
			let mid = (low + high) / 2;
			let mid_val = self.vec[mid].id();
			match mid_val.cmp(&id) {
				Ordering::Equal => {
					self.cache.insert(id, mid);
					return Some(mid);
				},
				Ordering::Greater => high = mid - 1,
				Ordering::Less => low = mid + 1,
			}
		}
		None
	}
	fn _get_insert_index(&self, id:String) -> usize {
		let mut low = 0;
		let mut high = self.vec.len() - 1;
		while low <= high {
			let mid = (low + high) / 2;
			let mid_val = self.vec[mid].id();
			match mid_val.cmp(&id) {
				Ordering::Equal => return mid,
				Ordering::Greater => high = mid - 1,
				Ordering::Less => low = mid + 1,
			}
		}
		low
	}
	pub fn get(&mut self, id: String) -> Option<&mut GameEntity> {
		let index = self.get_index(id);
		if index.is_some() {
			return Some(&mut self.vec[index.unwrap()]);
		}
		None
	}
	fn clear_cache(&mut self) {
		self.cache.clear();
	}
	pub fn _insert(&mut self, item: GameEntity) {
		self.clear_cache();
		let id = item.id();
		let index = self._get_insert_index(id);
		self.vec.insert(index, item);
	}
	pub fn _remove(&mut self, id: String) -> Option<GameEntity> {
		self.clear_cache();
		let index = self.get_index(id);
		if index.is_some() {
			return Some(self.vec.remove(index.unwrap()));
		}
		None
	}
	pub fn iter(&self) -> std::slice::Iter<GameEntity> {
		self.vec.iter()
	}
	pub fn iter_mut(&mut self) -> std::slice::IterMut<GameEntity> {
		self.vec.iter_mut()
	}
	pub fn clear(&mut self) {
		self.vec.clear();
		self.clear_cache();
	}
}