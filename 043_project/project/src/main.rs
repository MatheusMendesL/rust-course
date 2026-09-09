#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String
}

impl<T> ChatMessage<T> {
    fn new(content: T, time: String) -> Self {
        Self {
            content,
            time,
        }
    }

    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self){
        println!("Watching the {:?}", self.content);
    }
}


fn main() {
    let chat = ChatMessage::new(
        "Hello, how are you?",
        "2026-12-02".to_string()
    );
    let chat2 = ChatMessage::new(
        "I am fine and you?".to_string(),
        "2026-12-02".to_string()
    );
    let chat3 = ChatMessage::new(
        DigitalContent::VideoFile,
        "2026-12-02".to_string()
    );

    chat3.consume_entertainment();

    println!("{:?}" ,chat.retrieve_time());
    println!("{:?}" ,chat2.retrieve_time());
    println!("{:?}" ,chat3.retrieve_time());
}
