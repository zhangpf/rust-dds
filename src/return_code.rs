
pub enum ReturnCode {
	Ok = 0,
	Error = 1,
	BadParameter = 2,
	Unsupported = 3,
	AlreadyDeleted = 4,
	OutOfResources = 5,
	NotEnabled = 6,
	ImmutablePolicy = 7,
	InconsistentPolicy = 8,
	PreconditionNotMet = 9,
	Timeout = 10,
	IllegalOperation = 11,
	NoData = 12
}