use std::{
    net::{SocketAddr, UdpSocket},
    time::Duration,
};

use clap::{App, Arg};
use trust_dns::{
    op::{Message, MessageType, OpCode, Query},
    rr::{Name, RecordType},
    serialize::binary::{BinEncodable, BinEncoder},
};

fn main() {
    const DOMAIN_TAG: &str = "domain";
    const DNS_SERVER_TAG: &str = "dns_server";

    // TODO: move into its own func
    let args = App::new("ip-resolver")
        .about("A simple IP resolver")
        .arg(Arg::with_name(DOMAIN_TAG).required(true))
        .arg(Arg::with_name(DNS_SERVER_TAG).default_value("1.1.1.1"))
        .get_matches();
    let domain_raw = args.value_of(DOMAIN_TAG).unwrap();
    let dns_raw = args.value_of(DNS_SERVER_TAG).unwrap();

    let domain = Name::from_ascii(&domain_raw).unwrap();
    let dns_server: SocketAddr = format!("{}:53", dns_raw).parse().expect("Unvalid address");

    let mut request_as_bytes: Vec<u8> = Vec::with_capacity(512);
    let mut response_as_bytes: Vec<u8> = vec![0; 512];

    // create DNS message
    let mut message = Message::new();
    message
        .set_id(rand::random::<u16>())
        .set_message_type(MessageType::Query)
        .add_query(Query::query(domain, RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    // encode message to bytes
    let mut encoder = BinEncoder::new(&mut request_as_bytes);
    message.emit(&mut encoder).unwrap();

    // Create a udp socker
    let localhost = UdpSocket::bind("0.0.0.0:0").expect("cannot bind to local socket");
    let timeout = Duration::from_secs(3);
    localhost.set_read_timeout(Some(timeout)).unwrap();
    localhost.set_nonblocking(false).unwrap();

    // send the request
    let _bytes = localhost
        .send_to(&request_as_bytes, &dns_server)
        .expect("socket misconfigured");

    // recieve the response_as_bytes
    let (_bytes, _addr) = localhost
        .recv_from(&mut response_as_bytes)
        .expect("timeout reaced");

    // parse the response
    let dns_message = Message::from_vec(&response_as_bytes).expect("unable to parse the response");
    for answer in dns_message.answers() {
        if answer.record_type() == RecordType::A {
            let ressource = answer.rdata();
            let ip = ressource
                .to_ip_addr()
                .expect("invalid IP address")
                .to_string();

            println!("{}", ip);
        }
    }
}
