use types::*;
use std::time::duration::Duration;
use entity::Entity;
use std::hash::{Writer};
use std::default::Default;
use std::i32;
// use types::DEFAULT_OCTET_SEQ;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct UserDataQos {
	value: OctetSeq,
}

impl UserDataQos {
	pub fn new(value: OctetSeq) -> UserDataQos {
		UserDataQos { value: value }
	}
}

impl Validation for UserDataQos {
	fn is_valid(&self) -> bool { self.value.is_valid() }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct TopicDataQos {
	value: OctetSeq,
}

impl TopicDataQos {
	pub fn new(value: OctetSeq) -> TopicDataQos {
		TopicDataQos { value: value }
	}
}

impl Validation for TopicDataQos {
	fn is_valid(&self) -> bool { self.value.is_valid() }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct GroupDataQos {
	value: OctetSeq,
}

impl GroupDataQos {
	pub fn new(value: OctetSeq) -> GroupDataQos {
		GroupDataQos { value: value }
	}
}

impl Validation for GroupDataQos {
	fn is_valid(&self) -> bool { self.value.is_valid() }
}

#[derive(Clone, PartialEq, Debug)]
enum DurabilityQosKind {
	Volatile = 0,
	TransientLocal = 1,
	TransientDurability = 2,
	PersistentDurability = 3,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DurabilityQos {
	kind: DurabilityQosKind,
}

impl Default for DurabilityQos {
	fn default() -> DurabilityQos { 
		DurabilityQos{ kind: DurabilityQosKind::Volatile }
	}
}

impl Validation for DurabilityQos {
	fn is_valid(&self) -> bool {
		match self.kind.clone() as i32 {
			0...3 => true,
			_ => false
		}
	}
}

#[derive(Clone, PartialEq, Debug)]
enum HistoryQosKind {
	KeepLast = 0,
	KeepAll = 1,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DurabilityServiceQos {
	service_cleanup_delay: Duration,
	history_kind: HistoryQosKind,
	history_depth: i32,
	max_samples: i32,
	max_instances: i32,
	max_samples_per_instance: i32,
}

impl Default for DurabilityServiceQos {
	fn default() -> DurabilityServiceQos { 
		DurabilityServiceQos { 
			service_cleanup_delay: Duration::zero(),
			history_kind: HistoryQosKind::KeepLast,
			history_depth: 1,
			max_samples: i32::MAX,
			max_instances: i32::MAX,
			max_samples_per_instance: i32::MAX,
		}
	}
}

impl Validation for DurabilityServiceQos {
	fn is_valid(&self) -> bool {
		true
	}
}

#[derive(Clone, PartialEq, Debug)]
enum PresentationQosAccessScopeKind {
    Instance = 0,
    Topic = 1,
    Group = 2,
}

#[derive(Clone, PartialEq, Debug)]
pub struct PresentationQos {
	access_scope: PresentationQosAccessScopeKind,
	coherent_access: bool,
	ordered_access: bool,	
}

impl Default for PresentationQos {
	fn default() -> PresentationQos {
		PresentationQos {
			access_scope: PresentationQosAccessScopeKind::Instance,
			coherent_access: false,
			ordered_access: false,
		}
	} 
}

impl Validation for PresentationQos {
	fn is_valid(&self) -> bool {
		true
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct DeadlineQos {
	period: Duration,
}

impl Default for DeadlineQos {
	fn default() -> DeadlineQos {
		DeadlineQos {
			period: Duration::max_value(),
		}
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct LatencyBudgetQos {
	duration: Duration,
}

impl Default for LatencyBudgetQos {
	fn default() -> LatencyBudgetQos {
		LatencyBudgetQos {
			duration: Duration::zero(),
		}
	}
}

#[derive(Clone, PartialEq, Debug)]
enum OwnershipQosKind {
	Shared = 0,
	Exclusive = 1,
}

#[derive(Clone, PartialEq, Debug)]
pub struct OwnershipQos {
	kind: OwnershipQosKind,
}

impl Default for OwnershipQos {
	fn default() -> OwnershipQos {
		OwnershipQos {
			kind: OwnershipQosKind::Shared,
		}
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct OwnershipStrengthQos {
	value: isize,
}

impl Default for OwnershipStrengthQos {
	fn default() -> OwnershipStrengthQos {
		OwnershipStrengthQos {
			value: 0,
		}
	}
}

#[derive(Clone, PartialEq, Debug)]
enum LivelinessQosKind {
	Automatic = 0,
	ManualByParticiant = 1,
	ManualByTopic = 2,
}

#[derive(Clone, PartialEq, Debug)]
pub struct LivelinessQos {
	kind: LivelinessQosKind,
	lease_duration: Duration,
}

impl Default for LivelinessQos {
	fn default() -> LivelinessQos {
		LivelinessQos {
			kind: LivelinessQosKind::Automatic,
			lease_duration: Duration::max_value(),
		}
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct TimeBasedFilterQos {
	minimum_separation: Duration,
}

impl Default for TimeBasedFilterQos {
	fn default() -> TimeBasedFilterQos {
		TimeBasedFilterQos {
			minimum_separation: Duration::zero(),
		}
	}
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct PartitionQos {
	name: Vec<String>,
}

impl Validation for PartitionQos {
	fn is_valid(&self) -> bool {
		true
	}
}

#[derive(Clone, PartialEq, Debug)]
enum ReliabilityQosKind {
	BestEffort = 0,
	Reliable = 1,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ReliabilityQos {
	kind: ReliabilityQosKind,
	max_blocking_time: Duration,
}

impl Default for ReliabilityQos {
	fn default() -> ReliabilityQos {
		ReliabilityQos {
			kind: ReliabilityQosKind::Reliable,
			max_blocking_time: Duration::milliseconds(100),
		}
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct TransportPriorityQos {
	value: i32,	
}


impl Default for TransportPriorityQos {
	fn default() -> TransportPriorityQos {
		TransportPriorityQos {
			value: 0,
		}
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct LifespanQos {
	duration: Duration,
}

impl Default for LifespanQos {
	fn default() -> LifespanQos {
		LifespanQos {
			duration: Duration::max_value(),
		}
	}
}

#[derive(Clone, PartialEq, Debug)]
enum DestinationOrderQosKind {
	ByReceptionTimestamp = 0,
	BySourceTimestamp = 1,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DestinationOrderQos {
	kind: DestinationOrderQosKind,
}

impl Default for DestinationOrderQos {
	fn default() -> DestinationOrderQos {
		DestinationOrderQos {
			kind: DestinationOrderQosKind::ByReceptionTimestamp,
		}
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct HistoryQos {
	kind: HistoryQosKind,
	depth: i32,
}

impl Default for HistoryQos {
	fn default() -> HistoryQos {
		HistoryQos {
			kind: HistoryQosKind::KeepLast,
			depth: 1,
		}
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct ResourceLimitsQos {
	max_samples: i32,
	max_instances: i32,
	max_samples_per_instance: i32,
}

impl Default for ResourceLimitsQos {
	fn default() -> ResourceLimitsQos {
		ResourceLimitsQos {
			max_samples: i32::MAX,
			max_instances: i32::MAX,
			max_samples_per_instance: i32::MAX,
		}
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct EntityFactoryQos {
	autoenable_created_entities: bool,
}

impl EntityFactoryQos {
	pub fn get_autoenable_created_entities(&self) -> bool {
		self.autoenable_created_entities
	}
}

impl EntityFactoryQos {
	pub fn new(
		autoenable_created_entities: bool
	) -> EntityFactoryQos {
		EntityFactoryQos {
			autoenable_created_entities: autoenable_created_entities,
		}
	}
}

impl Validation for EntityFactoryQos {
	fn is_valid(
		&self
	) -> bool { 
		if self.autoenable_created_entities == false {
			true
		} else if self.autoenable_created_entities == true {
			true
		} else {
			false
		}
	}
}

impl Default for EntityFactoryQos {
	fn default() -> EntityFactoryQos {
		EntityFactoryQos { autoenable_created_entities: true }
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct WriterDataLifecycleQos {
	autodispose_unregistered_instances: bool,
}

impl Default for WriterDataLifecycleQos {
	fn default() -> WriterDataLifecycleQos {
		WriterDataLifecycleQos { autodispose_unregistered_instances: true }
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct ReaderDataLifecycleQos {
	autopurge_suspended_samples_delay: Duration,
	autopurge_disposed_samples_delay: Duration,
}

impl Default for ReaderDataLifecycleQos {
	fn default() -> ReaderDataLifecycleQos {
		ReaderDataLifecycleQos { 
			autopurge_suspended_samples_delay: Duration::max_value(),
			autopurge_disposed_samples_delay: Duration::max_value(),
		}
	}
}
