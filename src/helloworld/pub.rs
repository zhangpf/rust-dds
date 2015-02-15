extern crate dds;

use std::default::Default;
use dds::dcps::FooTypeSupport;
use dds::api::DataWriter;
use dds::dcps::TypeSupport;
use domain_participant::DomainParticipant
use dds::DOMAIN_ID_DEFAULT;
use dds::PARTICIPANT_QOS_DEFAULT;
use dds::err::UnsatisfiedLinkError;
use err::DDSError;

#[derive(Default)]
struct Msg {
	pub user_id: i32,
	pub message: String,
}

const TYPE_SUPPORT_IDL_TYPE_NAME: &'static str = "HelloWorldData::Msg";
const TYPE_SUPPORT_IDL_KET_LIST: &'static str = "userID";
const META_DESCRIPTOR: &'static str = 
	"<MetaData version=\"1.0.0\">\
		<Module name=\"HelloWorldData\">\
			<Struct name=\"Msg\">\
				<Member name=\"userID\"><Long/></Member>\
				<Member name=\"message\"><String/></Member>\
			</Struct>\
			</Module>\
			</MetaData>";

struct MsgTypeSupport {
	copy_cache: i32,
	dcps_type_support: TypeSupport
}

impl MsgTypeSupport {
	pub fn new() -> MsgTypeSupport {
		let dds_ts = TypeSupport::new(
			"HelloWorldData/MsgDataReaderImpl",
			"HelloWorldData/MsgDataReaderViewImpl",
			"HelloWorldData/MsgDataWriterImpl",
			"(LHelloWorldData/MsgTypeSupport;)V",
			"null",
			"null"
			);
		try!(FooTypeSupport::alloc(
				&dds_ts, TYPE_SUPPORT_IDL_KET_LIST,
				TYPE_SUPPORT_IDL_TYPE_NAME, META_DESCRIPTOR
			)
		);

		MsgTypeSupport {
			dcps_type_support: dds_ts,
			copy_cache: 0,
		}
	}
}

/*
 participant = dpf.create_participant(DOMAIN_ID_DEFAULT.value,
                             PARTICIPANT_QOS_DEFAULT.value, null, STATUS_MASK_NONE.value);
*/


fn main() {
	let dpf = DomainParticipantFactory::get_instance();
	let dp = dpf.create_participant(
		DOMAIN_ID_DEFAULT, PARTICIPANT_QOS_DEFAULT,
		None, STATUS_MASK_NONE);

	let partition_name: &'static str = "HelloWorld example";

	let msg_ts = MsgTypeSupport::new();
	match msg_ts.register_type(dp, msg_ts.get_type_name()) {
		
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


