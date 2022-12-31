use std::collections::HashMap;
use std::cmp::Ordering;
use specs::DenseVecStorage;
use specs::Component;
use specs_derive::Component;

use crate::entity_lib::Entity;

#[derive(Component)]
pub struct EntityStore {
	vec: Vec<Entity>,
	cache: HashMap<String, usize>,
}
impl EntityStore {
	pub fn from(mut vec: Vec<Entity>) -> Self {
		vec.sort_by(|a,b| a.id().cmp(&b.id()));
		Self {
			vec,
			cache: HashMap::new(),
		}
	}
	pub fn empty() -> Self {
		Self {
			vec: vec![],
			cache: HashMap::new()
		}
	}
	fn get_index_no_cache(&self, id:String) -> Option<usize> {
		match self.cache.get(&id) {
			Some(index) => Some(*index),
			None => {
				match self.vec.binary_search_by(|e| e.id().cmp(&id)) {
					Ok(index) => Some(index),
					Err(_) => None,
				}
			}
		}
		// let index = self.cache.get(&id);
		// if index.is_some() {
		// 	return Some(*index.unwrap());
		// }
		// let mut low = 0;
		// let mut high = self.vec.len() - 1;
		// while low <= high {
		// 	let mid = (low + high) / 2;
		// 	let mid_val = self.vec[mid].id();
		// 	match mid_val.cmp(&id) {
		// 		Ordering::Equal => {
		// 			return Some(mid);
		// 		},
		// 		Ordering::Greater => high = mid - 1,
		// 		Ordering::Less => low = mid + 1,
		// 	}
		// }
		// None
	}
	fn get_index(&mut self, id:String) -> Option<usize> {
		let index = self.get_index_no_cache(id.clone());
		if index.is_some() {
			self.cache.insert(id, index.unwrap());
		}
		index
	}
	fn _get_insert_index(&self, id: String) -> usize {
		match self.vec.binary_search_by(|e| e.id().cmp(&id)) {
			Ok(index) => index,
			Err(index) => index,
		}
		// let mut low = 0;
		// let mut high = self.vec.len() - 1;
		// while low <= high {
		// 	let mid = (low + high) / 2;
		// 	let mid_val = self.vec[mid].id();
		// 	match mid_val.cmp(&id) {
		// 		Ordering::Equal => return mid,
		// 		Ordering::Greater => high = mid - 1,
		// 		Ordering::Less => low = mid + 1,
		// 	}
		// }
		// low
	}
	pub fn get_non_mut(&self, id: String) -> Option<&Entity> {
		let index = self.get_index_no_cache(id);
		if index.is_some() {
			return Some(&self.vec[index.unwrap()]);
		}
		None
	}
	pub fn get(&mut self, id: String) -> Option<&mut Entity> {
		let index = self.get_index(id);
		if index.is_some() {
			return Some(&mut self.vec[index.unwrap()]);
		}
		None
	}
	fn clear_cache(&mut self) {
		self.cache.clear();
	}
	pub fn _insert(&mut self, item: Entity) {
		self.clear_cache();
		let id = item.id();
		let index = self._get_insert_index(id);
		self.vec.insert(index, item);
	}
	pub fn _remove(&mut self, id: String) -> Option<Entity> {
		self.clear_cache();
		let index = self.get_index(id);
		if index.is_some() {
			return Some(self.vec.remove(index.unwrap()));
		}
		None
	}
	pub fn iter(&self) -> std::slice::Iter<Entity> {
		self.vec.iter()
	}
	pub fn iter_mut(&mut self) -> std::slice::IterMut<Entity> {
		self.vec.iter_mut()
	}
	pub fn clear(&mut self) {
		self.vec.clear();
		self.clear_cache();
	}
	// pub fn player(&mut self, player_id: String) -> &mut Player {
	// 	self.get(player_id.clone()).unwrap().mut_unwrap_player()
	// }
}