use edge_translate::Client;

pub fn main() {
    let mut client = Client::new();
    println!("{}", client.translate_to("Hello, world!", "zh-Hans").unwrap());
    println!("{}", client.translate_to("Hello, world!", "ja").unwrap());
    println!("{}", client.translate_to("你好，世界！", "en").unwrap());
}