
struct SampleInfo {
	sample_state: SampleStateKind,
	view_state: ViewStateKind,
	instance_state: InstanceStateKind,
	disposed_generation_count: i32,
	no_writers_generation_count: i32,
	sample_rank: i32,
	generation_rank: i32,
	absolute_generation_rank: i32,
	source_timestamp: Time,
	instance_handle: InstanceHandle,
	publication_handle: InstanceHandle,
	valid_data: bool
}

