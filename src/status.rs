use types::InstanceHandle;

pub struct SampleLostStatus {
	total_count: isize,
	total_count_change: isize,
}

enum SampleRejectedStatusKind {
	NotRejected = 0, 
	RejectedByInstancesLimit = 1,
	RejectedBySamplesLimit = 2,
	RejectedBySamplesPerInstanceLimit = 3,
}

pub struct SampleRejectedStatus {
	total_count: isize,
	total_count_change: isize,
	last_reason: SampleRejectedStatusKind,
	last_instance_handle: InstanceHandle,
}

pub struct LivelinessLostStatus {
	total_count: isize,
	total_count_change: isize,
	alive_count_change: isize,
	not_alive_count_change: isize,
	last_publication_handle: InstanceHandle,
}

pub struct RequestedDeadlineMissedStatus {
	total_count: isize,
	total_count_change: isize,
	last_instance_handle: isize,
}

pub struct RequestedIncompatibleQosStatus {
	total_count: isize,
	total_count_change: isize,
	last_policy_id: isize,
}

pub struct PublicationMatchedStatus {
	total_count: isize,
	total_count_change: isize,
	current_count: isize,
	last_subscription_handle: InstanceHandle,
}

pub struct SubscriptionMatchedStatus {
	total_count: isize,
	total_count_change: isize,
	current_count: isize,
	current_count_change: isize,
	last_publication_handle: InstanceHandle,
}

pub struct InconsistentTopicStatus {
	total_count: u32,
	total_count_change: u32,
}

impl InconsistentTopicStatus {
	pub fn new(
		total_count: u32, total_count_change: u32
	) -> InconsistentTopicStatus {
		InconsistentTopicStatus {
			total_count: total_count,
			total_count_change: total_count_change,
		}
	}
}

pub struct LivelinessChangedStatus {
	alive_count: u32,
	not_alive_count: u32,
	alive_count_change: u32,
	not_alive_count_change: u32,
	last_publication_handle: u32,
}
