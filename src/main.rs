fn main() {
    let (mut socket, response) =
        tungstenite::connect(url::Url::parse("ws://127.0.0.1:5683").unwrap())
            .expect("No Ws Serve Found");
    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }
    
    loop {
        let msg=socket.read_message().expect("Error Reading Message");
        println!("Recive : {}",&msg);
        socket.write_message(tungstenite::Message::Text("Accept".to_string())).expect("Error Sending Message");
    }
}

