use err::DDSError;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::cell::RefMut;
use std::collections::HashSet;
use qos::EntityFactoryQos;
use domain_participant::DomainParticipant;
use domain_participant::DomainParticipantQos;
use domain_participant::DomainParticipantListener;
// use qos::DEFAULT_ENTITY_FACTORY_QOS;
use types::*;
use entity::EntityOperation;
use collections::CombiantionSet;
//use helper::*;
use std::default::Default;
use std::hash::{Hash, Hasher, Writer};

#[derive(Default, Clone, PartialEq, Debug)]
pub struct DomainParticipantFactoryQos {
	entity_factory: EntityFactoryQos,
}

impl DomainParticipantFactoryQos {
	pub fn new(
		entity_factory: EntityFactoryQos
	) -> DomainParticipantFactoryQos {
		DomainParticipantFactoryQos {
			entity_factory: entity_factory,
		}
	}

	pub fn is_autoenable_created_entities(
		&self
	) -> bool {
		self.entity_factory.get_autoenable_created_entities()
	}
}

impl EntityQos for DomainParticipantFactoryQos {
	fn is_consistent(&self) -> bool { self.entity_factory.is_valid() }

	fn is_mutability_compatible(
		&self, other: DomainParticipantFactoryQos
	) -> bool {
		true
	}
}

impl<H: Hasher + Writer> Hash<H> for DomainParticipantFactoryQos {
	fn hash(&self, state: &mut H) {
		unsafe {
			(self as *const DomainParticipantFactoryQos as u64).hash(state);
		}
	}
}

#[derive(Default)]
pub struct DomainParticipantFactory {
	default_dp_qos: DomainParticipantQos,
	dpf_qos: DomainParticipantFactoryQos,
	participants: CombiantionSet<DomainId, Rawlink<DomainParticipant>>,
}

impl DomainParticipantFactory {
	pub fn get_instance<'a>() -> &'a mut DomainParticipantFactory {
		unsafe {
			&mut *THE_DOMAIN_PARTICIPANT_FACTORY
		}
	}

	pub fn create_participant(
		&mut self, domain_id: DomainId, 
		dp_qos: Option<DomainParticipantQos>, 
		dp_listener: Option<DomainParticipantListener>, mask: u32
	) -> Result<Box<DomainParticipant>, DDSError> {

		let qos = match dp_qos {
			None => self.get_default_participant_qos(),
			Some(v) => v,
		};
		if !qos.is_consistent() {
			return Err(DDSError::InconsistentPolicy);
		}

		let mut dp = Box::new(DomainParticipant::new(domain_id, qos, dp_listener));
		if self.dpf_qos.is_autoenable_created_entities() {
			dp.enable();
		}
		self.participants.insert(domain_id, Rawlink::some(&*dp));

		Ok(dp)
	}

	pub fn delete_participant(
		&mut self, dp: &DomainParticipant
	) -> DDSError {
		if !dp.is_empty() {
			return DDSError::PreconditionNotMet;
		}

		match self.participants.remove_by_second(&Rawlink::some(&dp)) {
			DDSError::Ok => DDSError::Ok,
			_ => DDSError::BadParameter,
		}
	}

	pub fn lookup_participant(&self, domain_id: DomainId) 
		-> Option<&mut DomainParticipant> {
		
		 match self.participants.get_second_by_first(&domain_id) {
		 	None => None,
		 	Some(ref v) => v.resolve()
		 }
	}

	pub fn get_default_participant_qos(&self) 
		-> DomainParticipantQos {
		self.default_dp_qos.clone()
	}

	pub fn set_default_participant_qos(
		&mut self, dp_qos: DomainParticipantQos
	) -> DDSError {
		
		if !dp_qos.is_consistent() {
			return DDSError::InconsistentPolicy;
		}
		self.default_dp_qos = dp_qos.clone();
		
		DDSError::Ok
	}

	pub fn set_qos(
		&mut self, dpf_qos: DomainParticipantFactoryQos
	) -> DDSError {
		
		if !dpf_qos.is_consistent() {
			return DDSError::InconsistentPolicy;
		}

		if !self.dpf_qos.is_mutability_compatible(dpf_qos.clone()) {
			return DDSError::ImmutablePolicy;
		}

		self.dpf_qos = dpf_qos.clone();

		DDSError::Ok
	}

	pub fn get_qos(&self) -> DomainParticipantFactoryQos {
		self.dpf_qos.clone()
	}
}

lazy_static! {

	static mut ref THE_DOMAIN_PARTICIPANT_FACTORY: DomainParticipantFactory = 
		Default::default();
}

#[cfg(test)]
mod tests {
	use domain_participant_factory::*;
	use domain_participant::*;
	use std::default::Default;
	use qos::*;
	use err::*;
	use types::*;
	use entity::*;
	use std::time::Duration;

	#[test]
	pub fn test_default_participant_qos() {		

		let factory: DomainParticipantFactory = Default::default();
		let default_dp_qos: DomainParticipantQos = Default::default(); 
		assert_eq!(
			factory.get_default_participant_qos(),
			default_dp_qos
		);
	}

	#[test]
	pub fn test_set_and_get_default_participant_qos() {

		let mut factory: DomainParticipantFactory = Default::default();
		let dp_qos_1 = DomainParticipantQos::new(
			UserDataQos::new(
				OctetSeq::new(
					10, vec![32, 43, 33]
				)
			),
			Default::default(),
		);

		let res1 = factory.set_default_participant_qos(&dp_qos_1);
		assert_eq!(res1, DDSError::Ok);

		assert_eq!(factory.get_default_participant_qos(), dp_qos_1);

		let dp_qos_2 = DomainParticipantQos::new(
			UserDataQos::new(
				OctetSeq::new(
					2, vec![32, 43, 33]
				)
			),
			EntityFactoryQos::new(false),
		);

		let res2 = factory.set_default_participant_qos(&dp_qos_2);
		assert_eq!(res2, DDSError::InconsistentPolicy);
	}

	#[test]
	pub fn test_set_and_factory_qos() {
		let mut factory: DomainParticipantFactory = Default::default();

		let default_dpf_qos: DomainParticipantFactoryQos = Default::default();
		assert_eq!(default_dpf_qos, factory.get_qos());

		let dpf_qos_1 = DomainParticipantFactoryQos::new(
			EntityFactoryQos::new(false)
		);

		let res1 = factory.set_qos(&dpf_qos_1);
		assert_eq!(dpf_qos_1, factory.get_qos());
	}

	#[test]
	pub fn test_create_participant() {
		let mut factory: DomainParticipantFactory = Default::default();

		let res_1 = factory.create_participant(3, None, None, 0);
		assert!(res_1.is_ok());
		
		let dp_1 = res_1.unwrap();
		assert_eq!(dp_1.get_qos(), factory.get_default_participant_qos());

		let dp_qos = DomainParticipantQos::new(
			UserDataQos::new(
				OctetSeq::new(
					4, vec![32, 43, 33]
				)
			),
			EntityFactoryQos::new(false),
		);
		let res_2 = factory.create_participant(4, Some(dp_qos.clone()), None, 0);
		//println!("{}", res_2.err().expect(""))'
		assert!(res_2.is_ok());

		let dp_2 = res_2.unwrap();
		assert_eq!(dp_2.get_qos(), dp_qos);
	}

	#[test]
	pub fn test_delete_participant() {
		let mut factory: DomainParticipantFactory = Default::default();

		let res_1 = factory.create_participant(3, None, None, 0);
		assert!(res_1.is_ok());
		
		let dp = res_1.unwrap();
		let res_2 = factory.delete_participant(&*dp);
		assert_eq!(res_2, DDSError::Ok);
	}

	#[test]
	pub fn test_lookup_participant() {
		let mut factory: DomainParticipantFactory = Default::default();

		let res_1 = factory.create_participant(3, None, None, 0);
		assert!(res_1.is_ok());
		
		let dp = res_1.unwrap();
		let res_2 = factory.lookup_participant(3);
		assert!(res_2.is_some());
		assert_eq!(*dp, *res_2.unwrap());
	}

	#[test]
	pub fn test_all() {
		let mut factory: DomainParticipantFactory = Default::default();

		let dp_qos = DomainParticipantQos::new(
			UserDataQos::new(
				OctetSeq::new(
					4, vec![32, 43, 33]
				)
			),
			EntityFactoryQos::new(false),
		);
		let res_1 = factory.create_participant(3, None, None, 0);
		let res_2 = factory.create_participant(4, Some(dp_qos.clone()), None, 0);
		assert!(res_1.is_ok());
		assert!(res_2.is_ok());

		factory.set_default_participant_qos(&dp_qos);
		let res_3 = factory.create_participant(5, None, None, 0);
		assert!(res_3.is_ok());
		assert_eq!(factory.get_default_participant_qos(), dp_qos);

		let dp_1 = res_1.unwrap();
		let dp_2 = res_2.unwrap();
		let dp_3 = res_3.unwrap();
		assert_eq!(*factory.lookup_participant(3).unwrap(), *dp_1);
		assert_eq!(*factory.lookup_participant(4).unwrap(), *dp_2);
		assert_eq!(*factory.lookup_participant(5).unwrap(), *dp_3);

		let default_dp_qos: DomainParticipantQos = Default::default(); 
		assert_eq!(dp_1.get_qos(), default_dp_qos);
		assert_eq!(dp_3.get_qos(), dp_qos);
		assert_eq!(dp_2.get_qos(), dp_qos);
		assert_eq!(factory.delete_participant(&dp_1), DDSError::Ok);
		assert_eq!(factory.delete_participant(&dp_1), DDSError::BadParameter);
		assert_eq!(factory.delete_participant(&dp_2), DDSError::Ok);
		assert_eq!(factory.delete_participant(&dp_3), DDSError::Ok);
	}
}
