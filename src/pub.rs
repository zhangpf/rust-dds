extern crate dds;

use std::default::Default;
use dds::dcps::FooDataWriterImpl;
use dds::api::DataWriter;

#[derive(Default)]
struct Msg {
	pub user_id: i32,
	pub message: &str,
}

mod msg_type_support {
	const idl_type_name: 'static str = "HelloWorldData::Msg";
	const idl_key_list: 'static str = "userID";

	struct MsgTypeSupport {
		copy_cache: isize,
	}

	impl MsgTypeSupport {
		pub fn new() -> {
			TypeSupport::new(
				"HelloWorldData/MsgDataReaderImpl",
				"HelloWorldData/MsgDataReaderViewImpl",
				"HelloWorldData/MsgDataWriterImpl",
				"(LHelloWorldData/MsgTypeSupport;)V",
				"null",
				"null"
				)
			FooTypeSupportImpl.alloc(&self, )
		}
	}
}

impl TypeSupport for MsgTypeSupport {

}

impl TypeSupportOperation for MsgTypeSupport {

}

struct MsgDataWriter {
	copy_cache: isize,
	type_support: &MsgTypeSupport,
}

struct MsgHolder {
	pub value: &Msg,
}

impl DataWriter for MsgDataWriter {

}

impl<'a> MsgDataWriter<'a> {
	pub fn new(ts: &MsgTypeSupport) -> MsgDataWriter {
		MsgDataWriter {
			copy_cache: ts.get_copy_cache();
			type_support: ts,
		}
	}

	pub fn register_instance(&self, instance_data: Msg) -> isize {
		FooDataWriterImpl.register_instance(self, self.copy_cache,instance_data)
	}

	pub fn register_instance_w_timestamp(&self, instance_data: Msg, 
		source_timestamp: Time) -> isize {
		FooDataWriterImpl.register_instance_w_timestamp(self, 
			self.copy_cache, instance_data, source_timestamp)
	}

	pub fn unregister_instance(&self, instance_data: Msg, handle: isize) -> i32 {
		FooDataWriterImpl.unregister_instance(self, self.copy_cache, instance_data, handle)
	}

	pub fn unregister_instance_w_timestamp(&self, instance_data: Msg, 
		handle: isize, source_timestamp: Time) -> i32 {
		FooDataWriterImpl.unregister_instance_w_timestamp(self, self.copy_cache, 
			instance_data, handle, source_timestamp)
	}

	pub fn write(&self, instance_data: Msg, handle: isize) -> i32 {
		FooDataWriterImpl.write(self, self.copy_cache, instance_data, handle)
	}

	pub fn write_w_timestamp(&self, instance_data: Msg, handle: isize, 
		source_timestamp: Time) -> i32 {
		FooDataWriterImpl.write_w_timestamp(self, self.copy_cache, 
			instance_data, handle, source_timestamp)
	}

	pub fn dispose(&self, instance_data: Msg, instance_handle: isize) -> i32 {
		FooDataWriterImpl.dispose(self, self.copy_cache, instance_data, instance_handle)
	}

	pub fn dispose_w_timestamp(&self, instance_data: Msg, instance_handle: isize,
		source_timestamp: Time) -> i32 {
		FooDataWriterImpl.dispose_w_timestamp(self, self.copy_cache, instance_handle, 
			source_timestamp)
	}

	pub fn writedispose(&self, instance_data: Msg, handle: isize) -> i32 {
		FooDataWriterImpl.writedispose(self, self.copy_cache, instance_data, handle)
	}

	pub fn writedispose_w_timestamp(&self, instance_data: Msg, instance_handle: isize,
		source_timestamp: Time) -> i32 {
		FooDataWriterImpl.writedispose_w_timestamp(self, self.copy_cache, instance_data, 
			instance_handle, source_timestamp)
	}

	pub fn get_key_value(&self, key_holder: MsgHolder, handle: isize) -> i32 {
		FooDataWriterImpl.get_key_value(self, self.copy_cache, key_holder, handle)
	}

	pub fn lookup_instance(&self, instance_data: Msg) -> isize {
		FooDataWriterImpl.lookup_instance(self, self.copy_cache, instance_data)
	}
}

