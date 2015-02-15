struct ReadCondition {

}

impl ReadCondition {
	pub fn get_datareader(&self) -> DataReader {

	}

	pub fn get_sample_state_mask(&self) -> SampleStateKind[] {

	}

	pub fn get_view_state_mask(&self) -> ViewStateKind[] {

	}

	pub fn get_instance_state_mask(&self) -> InstanceStateKind[] {

	}
}

struct QueryCondition {
	pub fn get_query_expression(&self) -> String {

	}
	
	pub fn get_query_parameters(&self,
		&mut query_parameters: String[]) -> ReturnCode {

	}

	pub fn set_query_parameters(&self,
		&mut query_parameters: String[]) -> ReturnCode {
	}
}
