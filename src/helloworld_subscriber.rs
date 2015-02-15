extern crate dds;

use dds::api::DomainParticipant;
use dds::api::Subscriber;
use dds::api::DataReader;
use dds::api::TypeSupport;

use ::DDS_sequence_HelloWorldData_Msg::*;

fn check_handle(ts: TypeSupport, msg: &'static str) {
	if 
}

void checkHandle(
    void *handle,
    const char *info ) {
     
     if (!handle) {
        fprintf(stderr, "\n Error in %s: Creation failed: invalid handle\n", info);
        exit (1);
     }
}


fn main() {
	let domain_participant = DomainParticipant::create_participant("HelloWorld example");
	let message_seq = sequence_helloworld_data_msg__alloc();	
	let message_ts = helloworld_data_msg_type_support__alloc();
	let message_topic = domain_participant.create_topic("HelloWorldData_Msg", )
	let subscriber = Subscriber::new();
	let data_reader = 
}