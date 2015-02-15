use err::DDSError;
use std::time::duration::Duration;
use qos::PresentationQos;
use qos::PartitionQos;
use qos::GroupDataQos;
use qos::EntityFactoryQos;
use data_writer::DataWriter;
use data_writer::DataWriterQos;
use data_writer::DataWriterListener;
use topic::Topic;
use std::default::Default;
use topic::TopicQos;
use domain_participant::DomainParticipant;
use types::*;
use entity::{EntityOperation, Entity};
use std::hash::{Hash, Hasher, Writer};


#[derive(Default, PartialEq, Debug, Clone)]
pub struct PublisherQos {
	presentation: PresentationQos,
	partition: PartitionQos,
	group_data: GroupDataQos,
	entity_factory: EntityFactoryQos,
}

impl EntityQos for PublisherQos {
	fn is_consistent(&self) -> bool {
		if !self.presentation.is_valid() { return false; }
		if !self.partition.is_valid() { return false; }
		if !self.group_data.is_valid() { return false; }
		if !self.entity_factory.is_valid() { return false; }
		true
	}

	fn is_mutability_compatible(
		&self, other: PublisherQos
	) -> bool {
		true
	}
}

impl<H: Hasher + Writer> Hash<H> for PublisherQos {
	fn hash(&self, state: &mut H) {
		unsafe {
			(self as *const PublisherQos as u64).hash(state);
		}
	}
}

#[derive(Debug)]
pub struct Publisher {
	parent_factory: Rawlink<DomainParticipant>,
	p_qos: PublisherQos,
	parent_entity: Entity,
}

impl PartialEq for Publisher {
	fn eq(&self, other: &Publisher) -> bool {
		unsafe { 
			(self as *const Publisher) == (other as *const Publisher)
		} 
	}

	fn ne(&self, other: &Publisher) -> bool {
		!self.eq(other)
	}
}

impl EntityOperation<
	PublisherQos, PublisherListener
> for Publisher {
	fn set_qos(
		&mut self, p_qos: PublisherQos
	) -> DDSError {
		if !p_qos.is_consistent() {
			return DDSError::InconsistentPolicy;
		}

		if self.is_enabled() && 
			!self.p_qos.is_mutability_compatible(p_qos.clone()) {
				return DDSError::ImmutablePolicy;
		}

		self.p_qos = p_qos.clone();
		DDSError::Ok
	}
	
	fn get_qos(&self) -> PublisherQos { self.p_qos.clone() }
	
	fn set_listener(
		&mut self, p_listener: Option<PublisherListener>, mask: u32
	) -> DDSError {
		DDSError::Error
	}
	
	fn get_listener(&self) -> Option<PublisherListener> {
		None
	}

	fn get_entity(&mut self) -> &mut Entity { &mut self.parent_entity }

	fn get_entity_immut(&self) -> &Entity { &self.parent_entity }
}


impl Publisher {

	pub fn new(
		p_qos: PublisherQos, p_listener: Option<PublisherListener>, 
		mask: u32, domain_participant: &DomainParticipant
	) -> Publisher {
		Publisher {
			parent_factory: Rawlink::some(domain_participant),
			p_qos: p_qos,
			parent_entity: Entity::new(),
		}
	}

	pub fn is_empty(&self) -> bool {
		false
	}

	pub fn get_factory(
		&self
	) -> Option<&DomainParticipant> {
		unsafe {
			self.parent_factory.resolve_immut()
		}
	}
	
	pub fn lookup_datawriter(
		&self, topic_name: String
	) -> Option<DataWriter> {
		None
	}

	pub fn create_datawriter(
		&self, a_topic: Topic, dw_qos: DataWriterQos, 
		a_listener: Option<DataWriterListener>, mask: u32
	) -> Option<DataWriter> {
		None
	}

	pub fn delete_datawriter(
		&self, dw: DataWriter
	) -> DDSError {
		DDSError::Error
	}

	pub fn suspend_publications(
		&self
	) -> DDSError {
		DDSError::Error
	}

	pub fn resume_publications(
		&self
	) -> DDSError {
		DDSError::Error
	}

	pub fn begin_coherent_changes(
		&self
	) -> DDSError {
		DDSError::Error
	}

	pub fn end_coherent_changes(
		&self
	) -> DDSError {
		DDSError::Error
	}

	pub fn wait_for_acknowledgments(
		&self, max_wait: Duration
	) -> DDSError {
		DDSError::Error
	}

	pub fn delete_contained_entities(
		&self
	) -> DDSError {
		DDSError::Error
	}

	pub fn set_default_datawriter_qos(
		&self, dw_qos: DataWriterQos
	) -> DDSError {
		DDSError::Error
	}

	pub fn get_default_datawriter_qos(
		&self
	) -> Result<DataWriterQos, DDSError> {
		Err(DDSError::Error)
	}

	pub fn copy_from_topic_qos(&self, dw_qos: DataWriterQos, t_qos: TopicQos) {
	}
 }

pub struct PublisherListener;

impl EntityListener for PublisherListener {}

lazy_static! {
	pub static ref DEFAULT_PUBLISHER_QOS: PublisherQos = Default::default();
}
