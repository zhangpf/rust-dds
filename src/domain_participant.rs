use err::DDSError;
use types::*;
use qos::UserDataQos;
use qos::EntityFactoryQos;
use publisher::{PublisherQos, PublisherListener, Publisher};
use subscriber::{SubscriberQos, SubscriberListener, Subscriber};
use topic::{Topic, TopicQos, TopicListener};
use topic::{ContentFilteredTopic, TopicDescription};
use topic::{TopicBuiltinTopicData, ParticipantBuiltinTopicData};
use std::default::Default;
use std::collections::HashSet;
use std::time::duration::Duration;
use entity::{Entity, EntityOperation};
use time;
use time::Tm;
use std::sync::mpsc::channel;
use std::old_io::timer::Timer;
use collections::CombiantionSet;
// use qos::DEFAULT_ENTITY_FACTORY_QOS;
// use qos::DEFAULT_USER_DATA_QOS;

#[derive(PartialEq, Debug)]
pub struct DomainParticipantListener;

impl EntityListener for DomainParticipantListener {}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct DomainParticipantQos {
	user_data: UserDataQos,	
	entity_factory: EntityFactoryQos,
}

impl DomainParticipantQos {
	pub fn new(
		user_data: UserDataQos, entity_factory: EntityFactoryQos
	) -> DomainParticipantQos {
		DomainParticipantQos {
			user_data: user_data,
			entity_factory: entity_factory,
		}
	}

	pub fn is_autoenable_created_entities(
		&self
	) -> bool {
		self.entity_factory.get_autoenable_created_entities()
	}
}

impl EntityQos for DomainParticipantQos {
	fn is_consistent(
		&self
	) -> bool {
		self.user_data.is_valid() & self.entity_factory.is_valid()
	}

	fn is_mutability_compatible(
		&self, other: DomainParticipantQos
	) -> bool { 
		true 
	}
}

#[derive(Debug, Default)]
pub struct DomainParticipant {
	parent_entity: Entity,
	domain_id: DomainId,
	dp_qos: DomainParticipantQos,
	dp_listener: Option<DomainParticipantListener>,
	publishers: HashSet<Rawlink<Publisher>>,
	subscribers: HashSet<Rawlink<Subscriber>>,
	topics: HashMap<String, Rawlink<Topic>>,
	cf_topics: HashMap<String, Rawlink<ContentFilteredTopic>>,
	default_s_qos: SubscriberQos,
	default_p_qos: PublisherQos,
	default_t_qos: TopicQos,

}

impl PartialEq for DomainParticipant {
	fn eq(&self, other: &DomainParticipant) -> bool {
		unsafe { 
			(self as *const DomainParticipant) == (other as *const DomainParticipant)
		} 
	}

	fn ne(&self, other: &DomainParticipant) -> bool {
		!self.eq(other)
	}
}


impl EntityOperation<
	DomainParticipantQos, DomainParticipantListener
> for DomainParticipant {
	fn set_qos(
		&mut self, qos: DomainParticipantQos
	) -> DDSError {
		DDSError::Error
	}
	
	fn get_qos(
		&self
	) -> DomainParticipantQos {
		self.dp_qos.clone()
	}
	
	fn set_listener(
		&mut self, dp_listener: Option<DomainParticipantListener>, mask: u32
	) -> DDSError {
		DDSError::Error
	}
	
	fn get_listener(&self) -> Option<DomainParticipantListener> {
		None
	}

	fn get_entity(&mut self) -> &mut Entity { &mut self.parent_entity }

	fn get_entity_immut(&self) -> &Entity { &self.parent_entity }
}

impl DomainParticipant {
	// called with qos inconsistent checked!!
	pub fn new(
		domain_id: DomainId, dp_qos: DomainParticipantQos,
		dp_listener: Option<DomainParticipantListener>
	) -> DomainParticipant {
		DomainParticipant {
			parent_entity: Entity::new(),
			domain_id: domain_id,
			dp_qos: dp_qos,
			dp_listener: dp_listener,
			publishers: HashSet::new(),
			subscribers: HashSet::new(),
			topics: HashSet::new(),
			//multi_topics: HashSet::new(),
			cf_topics: HashSet::new(),
			default_t_qos: Default::default(),
			default_p_qos: Default::default(),
			default_s_qos: Default::default(),
		}
	}

	pub fn is_empty(&self) -> bool {

		if !self.publishers.is_empty() { return false; }
		if !self.subscribers.is_empty() { return false; }
		if !self.topics.is_empty() { return false; }
		//if !self.multi_topics.is_empty() { return false; }
		if !self.cf_topics.is_empty() { return false; }
		true
	}

	pub fn create_publisher(
		&mut self, p_qos: Option<PublisherQos>,
		p_listener: Option<PublisherListener>, mask: u32
	) -> Result<Box<Publisher>, DDSError> {
		
		let qos = match p_qos {
			None => self.get_default_publisher_qos(),
			Some(v) => v,
		};
		if !qos.is_consistent() {
			return Err(DDSError::InconsistentPolicy);
		}

		let mut publisher = Box::new(
			Publisher::new(qos, p_listener, mask, self)
		);
		if self.dp_qos.is_autoenable_created_entities() {
			publisher.enable();
		}
		self.publishers.insert(Rawlink::some(&*publisher));

		Ok(publisher)
	}


	pub fn delete_publisher(
		&mut self, publisher: &Publisher
	) -> DDSError {
		
		match publisher.get_factory() {
			None => { return DDSError::PreconditionNotMet; },
			Some(v) if v != self => { return DDSError::PreconditionNotMet; }
			_ => {},
		};

		if !publisher.is_empty() {
			return DDSError::PreconditionNotMet;
		}

		match self.publishers.remove(&Rawlink::some(publisher)) {
			false => DDSError::Ok,
			_ => DDSError::BadParameter,
		}
	}

	pub fn create_subscriber(
		&mut self, s_qos: Option<SubscriberQos>,
		s_listener: Option<SubscriberListener>, mask: u32
	) -> Result<Box<Subscriber>, DDSError> {
		let qos = match s_qos {
			None => self.get_default_subscriber_qos(),
			Some(v) => v,
		};
		if !qos.is_consistent() {
			return Err(DDSError::InconsistentPolicy);
		}

		let mut subscriber = Box::new(
			Subscriber::new(qos, s_listener, mask, self)
		);
		if self.dp_qos.is_autoenable_created_entities() {
			subscriber.enable();
		}
		self.subscribers.insert(Rawlink::some(&*subscriber));

		Ok(subscriber)
	}

	pub fn delete_subscriber(
		&mut self, subscriber: &Subscriber
	) -> DDSError {
		
		match subscriber.get_factory() {
			None => { return DDSError::PreconditionNotMet; },
			Some(v) if v != self => { return DDSError::PreconditionNotMet; }
			_ => {},
		};

		if !subscriber.is_empty() {
			return DDSError::PreconditionNotMet;
		}

		match self.subscribers.remove(&Rawlink::some(subscriber)) {
			false => DDSError::Ok,
			_ => DDSError::BadParameter,
		}
	}

	pub fn create_topic(
		&mut self, topic_name: String, 
		type_name: String, t_qos: Option<TopicQos>,
		t_listener: Option<TopicListener>, mask: u32
	) -> Result<Box<Topic>, DDSError> {
		let qos = match t_qos {
			None => self.get_default_topic_qos(),
			Some(v) => v,
		};
		if !qos.is_consistent() {
			return Err(DDSError::InconsistentPolicy);
		}

		//TODO: use the type_name!!!

		let mut topic = Box::new(
			Topic::new(qos, topic_name, type_name, t_listener, mask, self)
		);
		if self.dp_qos.is_autoenable_created_entities() {
			topic.enable();
		}
		self.topics.insert(topic_name, Rawlink::some(&*topic));

		Ok(topic)
	} 

	pub fn delete_topic(
		&mut self, topic: &Topic
	) -> DDSError {

		match topic.get_factory() {
			None => { return DDSError::PreconditionNotMet; },
			Some(v) if v != self => { return DDSError::PreconditionNotMet; }
			_ => {},
		};

		if !topic.is_empty() {
			return DDSError::PreconditionNotMet;
		}

		match self.topics.remove(&Rawlink::some(topic)) {
			false => DDSError::Ok,
			_ => DDSError::BadParameter,
		}
	}

	pub fn create_content_filtered_topic(
		&mut self, name: String, related_topic: &Topic,
		filter_expression: String, expression_parameters: Vec<String>
	) -> Result<Box<ContentFilteredTopic>, DDSError> {
		let mut cf_topic = Box::new(
			ContentFilteredTopic::new(
				filter_expression, expression_parameters, related_topic, self
			)
		);

		self.cf_topics.insert(Rawlink::some(&*cf_topic));

		Ok(cf_topic)
	}

	pub fn delete_content_filtered_topic(
		&mut self, cf_topic: &ContentFilteredTopic
	) -> DDSError {
		match cf_topic.get_factory() {
			None => { return DDSError::PreconditionNotMet; },
			Some(v) if v != self => { return DDSError::PreconditionNotMet; }
			_ => {},
		};

		if !cf_topic.is_empty() {
			return DDSError::PreconditionNotMet;
		}

		match self.cf_topics.remove(&Rawlink::some(cf_topic)) {
			false => DDSError::Ok,
			_ => DDSError::BadParameter,
		}
	}

	// pub fn create_multi_topic(
	// 	&mut self, name: &str, type_name: &str, 
	// 	subscription_expression: &str, expression_parameters: &[&str]
	// ) -> Result<Box<MultiTopic>, DDSError> {
	// 	Err(DDSError::Ok)
	// }

	// pub fn delete_multi_topic(
	// 	&mut self, multi_topic: &MultiTopic
	// ) -> DDSError {
	// 	DDSError::Ok
	// }

	pub fn find_topic(
		&mut self, topic_name: String, timeout: Duration
	) -> Result<&Topic, DDSError> {
		// Need to be implemented.
		Err(DDSError::Error)
	} 

	pub fn lookup_topic_description(
		&mut self, name: String
	) -> Option<TopicDescription> {

	}

	pub fn get_builtin_subscriber(&self) -> Subscriber {
		None
	}

	pub fn ignore_participant(
		&self, handle: InstanceHandle
	) -> DDSError {
		DDSError::Ok
	} 

	pub fn ignore_topic(
		&self, handle: InstanceHandle
	) -> DDSError {
		DDSError::Ok
	}

	pub fn ignore_publication(
		&self, handle: InstanceHandle
	) -> DDSError {
		DDSError::Ok
	}

	pub fn ignore_subscription(
		&self, handle: InstanceHandle
	) -> DDSError {
		DDSError::Ok
	}

	pub fn get_domain_id(&self) -> DomainId { self.domain_id }

	pub fn delete_contained_entities(&self) -> DDSError {
		DDSError::Ok
	}

	pub fn assert_liveliness(&self) -> DDSError {
		DDSError::Ok
	}

	pub fn set_default_publisher_qos(
		&mut self, p_qos: &PublisherQos
	) -> DDSError {
		DDSError::Ok
	}

	pub fn get_default_publisher_qos(
		&self
	) -> PublisherQos {
		self.default_p_qos.clone()
	}

	pub fn set_default_subscriber_qos(
		&mut self, s_qos: &SubscriberQos
	) -> DDSError {
		DDSError::Ok
	}

	pub fn get_default_subscriber_qos(
		&self
	) -> SubscriberQos {
		self.default_s_qos.clone()
	}

	pub fn set_default_topic_qos(
		&mut self, t_qos: &TopicQos
	) -> DDSError {
		DDSError::Ok
	}

	pub fn get_default_topic_qos(
		&self
	) -> TopicQos {
		self.default_t_qos.clone()
	}

	pub fn get_discovered_participants(
		&self
	) -> Result<&[InstanceHandle], DDSError> {
		Err(DDSError::Ok)
	}  

	pub fn get_discovered_participant_data(
		&self, participant_handle: InstanceHandle
	) -> Result<ParticipantBuiltinTopicData, DDSError> {
		Err(DDSError::Ok)
	}

	pub fn get_discovered_topics(
		&self
	) -> Result<&[InstanceHandle], DDSError> {
		Err(DDSError::Ok)
	}

	pub fn get_discovered_topic_data(
		&self, topic_handle: InstanceHandle
	) -> Result<TopicBuiltinTopicData, DDSError> {
		Err(DDSError::Ok)
	}

	pub fn contains_entity(
		&self, handle: InstanceHandle
	) -> bool {
		false
	} 

	pub fn get_current_time(&self) -> Tm {
		time::now()
	}
}	

#[cfg(test)]
pub mod tests {
	use domain_participant::*;

}