use std::default::Default;
use std::ptr;
use std::hash::Hash;
use std::collections::hash_map::Hasher;
use std::old_io::Writer;
use std::mem;

#[allow(non_camel_case_types)]
pub type DomainId = u32;
pub type InstanceStateKind = u32;
pub type SampleStateKind = u32;
pub type ViewStateKind = u32;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct OctetSeq {
	max_len: usize,
	seq: Vec<u8>,
}

impl OctetSeq {
	pub fn new(
		max_len: usize, seq: Vec<u8>
	) -> OctetSeq {
		OctetSeq { max_len: max_len, seq: seq.clone() }
	}
}

impl Validation for OctetSeq {
	fn is_valid(&self) -> bool { self.seq.len() <= self.max_len }
}

// lazy_static! {
// 	pub static ref DEFAULT_OCTET_SEQ: OctetSeq = OctetSeq {
// 		seq: Vec::new(),
// 		max_len: 0,
// 	};
// }

pub struct OfferedDeadlineMissedStatus { 
	total_count: u32,
	total_count_change: u32,
	last_instance_handle: InstanceHandle,
}

pub struct OfferedIncompatibleQosStatus {
	total_count: u32,
	total_count_changeL: u32,
	last_qos_id: QosId, 
	qos_set: QosCountSeq,
}

pub struct StatusCondition;

pub struct StatusKind;

pub struct SubscriptionBuiltinTopicData;

pub struct SampleInfo;

pub struct ReadCondition;

pub struct QueryCondition;

pub struct PublicationBuiltinTopicData;

pub struct QosId;

pub struct QosCountSeq;

pub struct Data;

pub struct InstanceHandle;

pub struct Time;

pub fn check_qos_consistent<T>(
	qos_set: T
) -> bool {
	true
}

pub trait EntityQos {
	fn is_consistent(&self) -> bool;

	fn is_mutability_compatible(&self, Self) -> bool;
}

pub trait EntityListener {
	
}

pub trait Validation {
	fn is_valid(&self) -> bool;
}

#[derive(Debug)]
pub struct Rawlink<T> {
    p: *const T,
}

impl<T> Copy for Rawlink<T> {}
unsafe impl<T:'static+Send> Send for Rawlink<T> {}
unsafe impl<T:Send+Sync> Sync for Rawlink<T> {}

impl<T> PartialEq for Rawlink<T> {
	fn eq(&self, other: &Rawlink<T>) -> bool { unsafe { self.p == other.p } }

	fn ne(&self, other: &Rawlink<T>) -> bool { unsafe { self.p != other.p } }
}

impl<T> Eq for Rawlink<T> {}

impl<T> Hash<Hasher> for Rawlink<T> {
	fn hash(&self, state: &mut Hasher) {
        let addr = self.p as u64;
        addr.hash(state);
    }
}

/// Rawlink is a type like Option<T> but for holding a raw pointer
impl<T> Rawlink<T> {
    /// Like Option::None for Rawlink
    pub fn none() -> Rawlink<T> {
        Rawlink{p: ptr::null_mut()}
    }

    /// Like Option::Some for Rawlink
    pub fn some(n: &T) -> Rawlink<T> {
        Rawlink{p: n}
    }

    /// Convert the `Rawlink` into an Option value
    pub fn resolve_immut<'a>(&self) -> Option<&'a T> {
        if self.p.is_null() {
            None
        } else {
            Some(unsafe { mem::transmute(self.p) })
        }
    }

    /// Convert the `Rawlink` into an Option value
    pub fn resolve<'a>(&self) -> Option<&'a mut T> {
        if self.p.is_null() {
            None
        } else {
            Some(unsafe { mem::transmute(self.p) })
        }
    }

    /// Return the `Rawlink` and replace with `Rawlink::none()`
    fn take(&mut self) -> Rawlink<T> {
        mem::replace(self, Rawlink::none())
    }
}


#[cfg(test)]
mod tests {
}
