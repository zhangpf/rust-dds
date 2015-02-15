use err::DDSError;
use types::*;
use std::fmt::Debug;
use std::default::Default;
use std::hash::{Hash, Hasher};


pub trait EntityOperation<Q: EntityQos, L: EntityListener> {
	fn set_qos(&mut self, qos: Q) -> DDSError;
	
	fn get_qos(&self) -> Q;
	
	fn set_listener(&mut self, listener: Option<L>, mask: u32) -> DDSError;
	
	fn get_listener(&self) -> Option<L>;

	fn is_enabled(&self) -> bool { self.get_entity_immut().is_enabled() }

	fn enable(&mut self) { self.get_entity().enable(); }

	fn get_entity(&mut self) -> &mut Entity;

	fn get_entity_immut(&self) -> &Entity;
}

#[derive(PartialEq, Debug, Default, Hash)]
pub struct Entity {
	enabled: bool,
}

impl Entity {
	pub fn new() -> Entity {
		Entity {
			enabled: false,
		}
	}

	fn get_statuscondition(&self) -> Option<StatusCondition> {
		None
	}

	fn get_status_changes(&self) -> Option<StatusKind> {
		None
	}

	pub fn is_enabled(&self) -> bool { self.enabled }

	pub fn enable(&mut self) -> Option<DDSError> {
		self.enabled = true;
		None
	}

	fn get_instance_handle(&self) -> Option<InstanceHandle> {
		None
	}
}
