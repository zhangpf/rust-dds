
use std::default::Default;
use err::DDSError;
use qos::TopicDataQos;
use qos::DurabilityQos;
use qos::DurabilityServiceQos;
use qos::DeadlineQos;
use qos::LatencyBudgetQos;
use qos::LivelinessQos;
use qos::ReliabilityQos;
use qos::DestinationOrderQos;
use qos::HistoryQos;
use qos::ResourceLimitsQos;
use qos::TransportPriorityQos;
use qos::LifespanQos;
use qos::OwnershipQos;
use domain_participant::DomainParticipant;
use types::{EntityQos, Rawlink, EntityListener};
use entity::{Entity, EntityOperation};
use status::InconsistentTopicStatus;
use std::collections::HashSet;
use data_reader::DataReader;

#[derive(Debug)]
pub struct TopicDescription {
	name: String,
	type_name: String,
	parent_participant: Rawlink<DomainParticipant>
}

impl TopicDescription {

	pub fn new(
		name: String, type_name: String,
		domain_participant: &DomainParticipant
	) -> TopicDescription {
		TopicDescription {
			name: name,
			type_name: type_name,
			parent_participant: Rawlink::some(domain_participant),
		}
	}

	pub fn get_participant(
		&self
	) -> Option<&DomainParticipant> {
		unsafe {
			self.parent_participant.resolve_immut()
		}
	}
	
	pub fn get_type_name(&self) -> String {
		self.type_name.clone()
	}
	
	pub fn get_name(&self) -> String {
		self.name.clone()
	}
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct TopicQos {
	topic_data: TopicDataQos,
	durability: DurabilityQos,
	durability_service: DurabilityServiceQos,
	deadline: DeadlineQos,
	latency_budget: LatencyBudgetQos,
	liveliness: LivelinessQos,
	reliability: ReliabilityQos,
	destination_order: DestinationOrderQos,
	history: HistoryQos,
	resource_limits: ResourceLimitsQos,
	transport_priority: TransportPriorityQos,
	lifespan: LifespanQos,
	ownership: OwnershipQos,
}

impl EntityQos for TopicQos {
	fn is_consistent(&self) -> bool {
		true
	}

	fn is_mutability_compatible(
		&self, other: TopicQos
	) -> bool {
		true
	}
}

#[derive(Debug)]
pub struct Topic {
	parent_entity: Entity,
	parent_participant: Rawlink<DomainParticipant>,
	parent_description: TopicDescription,
	t_qos: TopicQos,
	inconsistent_total_count: u32,
	inconsistent_total_count_change: u32,
}

impl EntityOperation<TopicQos, TopicListener> for Topic {
	fn set_qos(
		&mut self, t_qos: TopicQos
	) -> DDSError {
		if !t_qos.is_consistent() {
			return DDSError::InconsistentPolicy;
		}

		if self.is_enabled() & 
			!self.t_qos.is_mutability_compatible(t_qos.clone()) {
				return DDSError::ImmutablePolicy;
			}
			
		
		self.t_qos = t_qos.clone();
		DDSError::Ok 
	}
	
	fn get_qos(&self) -> TopicQos { self.t_qos.clone() }
	
	fn set_listener(
		&mut self, listener: Option<TopicListener>, mask: u32
	) -> DDSError {
		DDSError::Ok
	}
	
	fn get_listener(
		&self
	) -> Option<TopicListener> {
		None
	}

	fn get_entity(&mut self) -> &mut Entity { &mut self.parent_entity }

	fn get_entity_immut(&self) -> &Entity { &self.parent_entity }
}

impl Topic {
	pub fn new(
		t_qos: TopicQos, topic_name: String, 
		type_name: String, t_listener: Option<TopicListener>,
		mask: u32, domain_participant: &DomainParticipant
	) -> Topic {
		Topic {
			parent_description: TopicDescription::new(
				topic_name, type_name, domain_participant),
			parent_participant: Rawlink::some(domain_participant),
			parent_entity: Entity::new(),
			t_qos: t_qos,
			inconsistent_total_count: 0,
			inconsistent_total_count_change: 0,
		}
	}

	pub fn is_empty(&self) -> bool { true }

	pub fn get_factory(
		&self
	) -> Option<&DomainParticipant> {
		unsafe {
			self.parent_participant.resolve_immut()
		}
	}

	pub fn get_inconsistent_topic_status(
		&mut self
	) -> Result<InconsistentTopicStatus, DDSError> {
		if !self.parent_entity.is_enabled() {
			return Err(DDSError::NotEnabled);
		}
		
		let change = self.inconsistent_total_count_change;
		self.inconsistent_total_count_change = 0;

		Ok(InconsistentTopicStatus::new(
			self.inconsistent_total_count,
			self.inconsistent_total_count_change
		))
	}

	pub fn get_name(
		&self
	) -> String { 
		self.parent_description.get_name()
	}

	pub fn get_type_name(
		&self
	) -> String {
		self.parent_description.get_type_name()
	}
}

#[derive(Debug)]
pub struct ContentFilteredTopic {
	expression: String,
	parameters: Vec<String>,
	related_topic: Rawlink<Topic>,
	parent_factory: Rawlink<DomainParticipant>,
	parent_description: TopicDescription,
	data_readers: HashSet<Rawlink<DataReader>>,
}

impl ContentFilteredTopic {
	pub fn new(
		expression: String, parameters: Vec<String>,
		related_topic: &Topic, domain_participant: &DomainParticipant
	) -> ContentFilteredTopic {
		ContentFilteredTopic {
			expression: expression,
			parameters: parameters,
			related_topic: Rawlink::some(related_topic),
			parent_factory: Rawlink::some(domain_participant),
			parent_description: TopicDescription::new(
				related_topic.get_name(),
				related_topic.get_type_name(),
				domain_participant
			),
			data_readers: HashSet::new(),
		}
	}

	pub fn is_empty(&self) -> bool {
		self.data_readers.is_empty()
	}

	pub fn get_factory(
		&self
	) -> Option<&DomainParticipant> {
		unsafe {
			self.parent_factory.resolve_immut()
		}
	}

	pub fn get_related_topic(
		&self
	) -> Option<&Topic> {
		unsafe {
			self.related_topic.resolve_immut()
		}
	}

	pub fn get_filter_expression(&self) -> String {
		self.expression.clone()
	}

	pub fn get_expression_parameters(
		&self
	) -> Vec<String> {
		self.parameters.clone()
	}	

	pub fn set_expression_parameters(
		&self, expression_parameters: Vec<String>
	) -> DDSError {
		DDSError::Unsupported
	}
}


pub struct TopicListener;

impl EntityListener for TopicListener {}

impl TopicListener {
	pub fn on_inconsistent_topic(
		&self, the_topic: Topic, status: InconsistentTopicStatus
	) {

	}
}

trait TypeSupport {
	fn register_type(
		&self, domain_participant: &DomainParticipant, type_name: String
	) -> DDSError {
		return domain_participant.register_type(type_name)
	}

	fn get_type_name(&self) -> String;
}

pub struct TopicBuiltinTopicData;
pub struct ParticipantBuiltinTopicData;
