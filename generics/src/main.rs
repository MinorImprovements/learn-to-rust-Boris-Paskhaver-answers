#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Type: {:?}", self.content);
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    let message1 = ChatMessage {
        content: "Hello",
        time: String::from("3"),
    };
    let message2 = ChatMessage {
        content: String::from("World"),
        time: String::from("5"),
    };
    let message3 = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("10"),
    };

    message3.consume_entertainment();

    println!("Time: {}", message1.retrieve_time());
    println!("Time: {}", message2.retrieve_time());
    println!("Time: {}", message3.retrieve_time());
}
