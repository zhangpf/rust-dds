use types::*;
use err::DDSError;
use topic::TopicDescription;
use subscriber::Subscriber;
use qos::DurabilityQos;
use qos::DeadlineQos;
use qos::LatencyBudgetQos;
use qos::LivelinessQos;
use qos::ReliabilityQos;
use qos::DestinationOrderQos;
use qos::HistoryQos;
use qos::ResourceLimitsQos;
use qos::UserDataQos;
use qos::OwnershipQos;
use qos::TimeBasedFilterQos;
use qos::ReaderDataLifecycleQos;
use std::time::duration::Duration;
use status::{SampleLostStatus, LivelinessChangedStatus};
use status::{RequestedDeadlineMissedStatus, RequestedIncompatibleQosStatus}; 
use status::{SampleRejectedStatus, SubscriptionMatchedStatus};
//use qos::SubscriptionKeyQos;
//use qos::ReaderLifespanQos;
//use qos::ShareQos;

pub struct DataReaderQos {
	durability: DurabilityQos,
	deadline: DeadlineQos,
    latency_budget: LatencyBudgetQos,
    liveliness: LivelinessQos,
    reliability: ReliabilityQos,
    destination_order: DestinationOrderQos,
    history: HistoryQos,
   	resource_limits: ResourceLimitsQos,
   	user_data: UserDataQos,
    ownership: OwnershipQos,
    time_based_filter: TimeBasedFilterQos,
    reader_data_lifecycle: ReaderDataLifecycleQos,
    //subscription_keys: SubscriptionKeyQos,
    //reader_lifespan: ReaderLifespanQos,
    //share: ShareQos,
}

#[derive(Debug)]
pub struct DataReader;

impl DataReader {
	
	pub fn read(
		&self, data_values: &mut Data, sample_infos: &mut [SampleInfo], 
		max_samples: i32, sample_states: &[SampleStateKind],
		view_states: &[ViewStateKind], instance_states: &[InstanceStateKind]
	) -> DDSError {
		DDSError::Error
	}

	pub fn take(
		&self, data_values: &mut [Data], sample_infos: &mut [SampleInfo], 
		max_samples: i32, sample_states: &[SampleStateKind],
		view_states: &[ViewStateKind], instance_states: &[InstanceStateKind]
	) -> DDSError {
		DDSError::Error
	}

	pub fn read_w_condition(
		&self, data_values: &mut [Data], sample_infos: &mut [SampleInfo], 
		max_samples: i32, a_condition: ReadCondition
	) -> DDSError {
		DDSError::Error
	}

	pub fn take_w_condition(
		&self, data_values: &mut [Data], sample_infos: &mut [SampleInfo],
		max_samples: i32, a_condition: ReadCondition
	) -> DDSError {
		DDSError::Error
	}

	pub fn read_next_sample(
		&self, data_values: &mut Data, sample_infos: &mut SampleInfo
	) -> DDSError {
		DDSError::Error
	}

	pub fn take_next_sample(
		&self, data_values: &mut Data, sample_infos: &mut SampleInfo, 
		max_samples: i32, previous_handle: InstanceHandle,
		sample_states: &[SampleStateKind], view_states: &[ViewStateKind],
		instance_states: &[InstanceStateKind]
	) -> DDSError {
		DDSError::Error
	}

	pub fn read_next_instance_w_condition(
		&self, data_values: &mut Data, sample_infos: &mut SampleInfo,
		max_samples: i32, previous_handle: InstanceHandle, condition: ReadCondition
	) -> DDSError {
		DDSError::Error
	}

	pub fn take_next_instance_w_condition(
		&self, data_values: &mut Data, sample_infos: &mut SampleInfo,
		max_samples: i32, previous_handle: InstanceHandle, condition: ReadCondition
	) -> DDSError {
		DDSError::Error
	}

	pub fn return_loan(
		&self, data_values: &mut Data, sample_infos: &mut SampleInfo
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

	pub fn create_readcondition(
		&self, sample_states: &[SampleStateKind],
		view_states: &[ViewStateKind], instance_states: &[InstanceStateKind]
	) -> DDSError {
		DDSError::Error
	}

	pub fn create_querycondition(
		&self, sample_states: &[SampleStateKind],
		view_states: &[ViewStateKind], instance_states: &[InstanceStateKind],
		query_expression: String, query_parameters: &[String]
	) -> QueryCondition {
		QueryCondition
	}

	pub fn delete_readcondition(
		&self, condition: ReadCondition
	) -> DDSError {
		DDSError::Error
	}

	pub fn get_liveliness_changed_status(
		&self, status: &mut LivelinessChangedStatus
	) -> DDSError {
		DDSError::Error
	}

	pub fn get_requested_deadline_missed_status(
		&self, status: &mut RequestedDeadlineMissedStatus
	) -> DDSError {
		DDSError::Error
	}

	pub fn get_requested_incompatible_qos_status(
		&self, status: &mut RequestedIncompatibleQosStatus
	) -> DDSError {
		DDSError::Error
	}

	pub fn get_sample_lost_status(
		&self, status: &mut SampleLostStatus
	) -> DDSError {
		DDSError::Error
	}

	pub fn get_sample_rejected_status(
		&self, status: &mut SampleRejectedStatus
	) -> DDSError {
		DDSError::Error
	}

	pub fn get_subscription_matched_status(
		&self, status: &mut SubscriptionMatchedStatus
	) -> DDSError {
		DDSError::Error
	}

	pub fn get_topicdescription(
		&self
	) -> Option<TopicDescription> {
		None
	}

	pub fn get_subscriber(
		&self
	) -> Option<Subscriber> {
		None
	}

	pub fn delete_contained_entities(
		&self
	) -> DDSError {
		DDSError::Error
	}

	pub fn wait_for_historical_data(
		&self, max_wait: Duration
	) -> DDSError {
		DDSError::Error
	}

	pub fn get_matched_publication_data(
		&self, publication_data: &mut PublicationBuiltinTopicData,
		publication_handle: InstanceHandle
	) -> DDSError {
		DDSError::Error
	}

	pub fn get_matched_publications(
		&self, publication_handles: &mut InstanceHandle
	) -> DDSError {
		DDSError::Error
	}
}

pub struct DataReaderListener;

