use kafka::consumer::{Consumer,FetchOffset};
use std::str;

struct Topic_Msg{
    name:String,
    message:String
}

fn main(){
    let msg = Topic_Msg{
        name:String::from("payment"),
        message:String::from("payment success")
    };
    
}


fn kafka_consumer(){
    let hosts:Vec<String> = vec![String::from("localhost:9092")];

    let mut consumer = Consumer::from_hosts(hosts)
    .with_topic(String::from("payment"))
    .with_fallback_offset(FetchOffset::Latest)
    .create()
    .unwrap();

    loop{
        for ms in consumer.poll().unwrap().iter(){
            for msg in ms.messages(){
                println!("{:?}", str::from_utf8(msg.value).unwrap());
            }

            consumer.consume_messageset(ms).unwrap();
        }
        consumer.commit_consumed().unwrap();
    }
}