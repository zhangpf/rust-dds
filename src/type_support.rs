
trait TypeSupport {
	pub fn register_type(&self, DomainParticipant, String) -> ReturnCode;
	pub fn get_type_name(&self) -> String;
}

