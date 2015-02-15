use err::DDSError;
use topic::TopicDescription;
use topic::TopicQos;
use types::InstanceStateKind;
use types::SampleStateKind;
use types::ViewStateKind;
use data_reader::DataReader;
use data_reader::DataReaderQos;
use data_reader::DataReaderListener;
use domain_participant::DomainParticipant;
use types::{EntityQos, Rawlink, EntityListener};
use entity::{EntityOperation, Entity};

use std::default::Default;

#[derive(PartialEq, Default, Debug, Clone)]
pub struct SubscriberQos;

impl EntityQos for SubscriberQos {
	fn is_consistent(&self) -> bool {
		true
	}

	fn is_mutability_compatible(&self, other: SubscriberQos) -> bool {
		true
	}
}

#[derive(Debug)]
pub struct Subscriber {
	parent_factory: Rawlink<DomainParticipant>,
	parent_entity: Entity,
	s_qos: SubscriberQos,
}

impl EntityOperation<
	SubscriberQos, SubscriberListener
> for Subscriber {
	fn set_qos(
		&mut self, s_qos: SubscriberQos
	) -> DDSError {
		if !s_qos.is_consistent() {
			return DDSError::InconsistentPolicy;
		}

		if self.is_enabled() && 
			!self.s_qos.is_mutability_compatible(s_qos.clone()) {
				return DDSError::ImmutablePolicy;
		}

		self.s_qos = s_qos.clone();
		DDSError::Ok
	}
	
	fn get_qos(&self) -> SubscriberQos { self.s_qos.clone() }
	
	fn set_listener(
		&mut self, s_listener: Option<SubscriberListener>, mask: u32
	) -> DDSError {
		DDSError::Error
	}
	
	fn get_listener(&self) -> Option<SubscriberListener> {
		None
	}

	fn get_entity(&mut self) -> &mut Entity { &mut self.parent_entity }

	fn get_entity_immut(&self) -> &Entity { &self.parent_entity }
}


impl Subscriber {
	pub fn new(
		s_qos: SubscriberQos, s_listener: Option<SubscriberListener>, 
		mask: u32, domain_participant: &DomainParticipant
	) -> Subscriber {
		Subscriber {
			parent_factory: Rawlink::some(domain_participant),
			parent_entity: Entity::new(),
			s_qos: s_qos,
		}
	}

	pub fn get_factory(&self) -> Option<&DomainParticipant> {
		unsafe {
			self.parent_factory.resolve_immut()
		}
	}

	pub fn is_empty(&self) -> bool {
		false
	}

	pub fn create_datareader(
		&self, topic: TopicDescription, dr_qos: Option<DataReaderQos>,
		dr_listener: Option<DataReaderListener>, mask: u32) 
	-> Option<DataReader> {
		None
	}

	pub fn delete_datareader(&self, data_reader: DataReader) -> DDSError {
		DDSError::Error
	}

	pub fn lookup_datareader(&self, topic_name: String) -> Option<DataReader> {
		None
	}

	pub fn begin_access(&self) -> DDSError {
		DDSError::Error
	}

	pub fn end_access(&self) -> DDSError {
		DDSError::Error
	}

	pub fn get_datareaders(
		&self, readers: &[DataReader], sample_states: &[SampleStateKind],
		view_states: &[ViewStateKind], instance_states: &[InstanceStateKind])
	-> Option<DataReader> {
		None
	}

	pub fn notify_datareaders(&self) -> DDSError {
		DDSError::Error
	}

	pub fn delete_contained_entities(&self) -> DDSError {
		DDSError::Error
	}

	pub fn set_default_datareader_qos(&self, dr_qos: DataReaderQos) -> DDSError {
		DDSError::Error
	}

	pub fn get_default_datareader_qos(&self) -> Result<DataReaderQos, DDSError> {
		Err(DDSError::Error)
	}

	pub fn copy_from_topic_qos(
		&self, a_datareader_qos: &mut [DataReaderQos], t_qos: &[TopicQos]) -> DDSError {
		DDSError::Error
	}
}


pub struct SubscriberListener;

impl EntityListener for SubscriberListener {}

lazy_static! {
	pub static ref DEFAULT_SUBSCRIBER_QOS: SubscriberQos = Default::default();
}
