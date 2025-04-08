use tokio::*;

#[tokio::main]
async fn main() {
    let mut x = std::thread::spawn(move || {
        println!("hello from spawned thread");
    });

    let network = read_from_network();

    select! {
        let network_stream <- network.await =>{
            // do something with stream
    }
    }

    Ok(())
}

fn read_from_network() {}

async fn foo() -> usize {
    println!("from foo");
    0
}

async fn foo2() {
    foo().await;
    println!("from foo2");
}
