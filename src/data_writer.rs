use qos::DeadlineQos;
use qos::LatencyBudgetQos;
use qos::LivelinessQos;
use qos::ReliabilityQos;
use qos::DestinationOrderQos;
use qos::HistoryQos;
use qos::ResourceLimitsQos;
use qos::TransportPriorityQos;
use qos::LifespanQos;
use qos::UserDataQos;
use qos::OwnershipQos;
use qos::OwnershipStrengthQos;
use qos::WriterDataLifecycleQos;
use types::*;
use std::time::duration::Duration;
use topic::Topic;
use publisher::Publisher;
use err::DDSError;
use status::{PublicationMatchedStatus, SubscriptionMatchedStatus};

pub struct DataWriterQos {
	deadline: DeadlineQos,
    latency_budget: LatencyBudgetQos,
    liveliness: LivelinessQos,
    reliability: ReliabilityQos,
    destination_order: DestinationOrderQos,
    history: HistoryQos,
   	resource_limits: ResourceLimitsQos,
    transport_priority: TransportPriorityQos,
    lifespan: LifespanQos,
    user_data: UserDataQos,
    ownership: OwnershipQos,
    ownership_strength: OwnershipStrengthQos,
    writer_data_lifecycle: WriterDataLifecycleQos,
}

pub struct DataWriter;

impl DataWriter {

	pub fn register_instance(
		&self, instance: Data
	) -> InstanceHandle {
		InstanceHandle
	}

	pub fn register_instance_w_timestamp(
		&self, instance: Data, timestamp: Time
	) -> InstanceHandle {
		InstanceHandle
	}

	pub fn unregister_instance(
		&self, instance: Data, handle: InstanceHandle
	) -> DDSError {
		DDSError::Error
	}

	pub fn unregister_instance_w_timestamp(
		&self, instance: Data, handle: InstanceHandle, timestamp: Time
	) -> DDSError {
		DDSError::Error
	}

	pub fn get_key_value(
		&self, key_holder: &mut Data, handle: InstanceHandle
	) -> DDSError {
		DDSError::Error
	}

	pub fn lookup_instance(
		&self, instance: Data
	) -> InstanceHandle {
		InstanceHandle
	}

	pub fn write(
		&self, data: Data, handle: InstanceHandle
	) -> DDSError {
		DDSError::Error
	}

	pub fn write_w_timestamp(
		&self, data: Data, handle: InstanceHandle
	) -> DDSError {
		DDSError::Error
	}

	pub fn dispose(
		&self, data: Data, handle: InstanceHandle
	) -> DDSError {
		DDSError::Error
	}

	pub fn dispose_w_timestamp(
		&self, data: Data, handle: InstanceHandle, timestamp: Time
	) -> DDSError {
		DDSError::Error
	}

	pub fn wait_for_acknowledgments(
		&self, max_wait: Duration
	) -> DDSError {
		DDSError::Error
	}

	pub fn get_liveliness_lost_status(
		&self, status: &mut OfferedDeadlineMissedStatus
	) -> DDSError {
		DDSError::Error
	}

	pub fn get_offered_incompatible_qos_status(
		&self, status: &mut OfferedIncompatibleQosStatus) -> DDSError {
		DDSError::Error
	}

	pub fn get_publication_matched_status(
		&self, status: &mut PublicationMatchedStatus) -> DDSError {
		DDSError::Error
	}

	pub fn get_topic(
		&self
	) -> Option<Topic> {
		None
	}

	pub fn get_publisher(
		&self
	) -> Option<Publisher> {
		None
	}

	pub fn assert_liveliness(
		&self
	) -> DDSError {
		DDSError::Error
	}

	pub fn get_matched_subscription_data(
		&self, subscription_data: &mut SubscriptionBuiltinTopicData,
		subscription_handle: InstanceHandle
	) -> DDSError {
		DDSError::Error	
	}

	pub fn get_matched_subscriptions(
		&self, subscription_handles: InstanceHandle
	) -> DDSError {
		DDSError::Error
	}
}


pub struct DataWriterListener;