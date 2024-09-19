use tokio::sync::mpsc;
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (sender, mut receiver) = mpsc::channel(32);

    // Создаем задачу, которая будет ожидать сообщения
    let handle = task::spawn(async move {
        while let Some(message) = receiver.recv().await {
            println!("Получено сообщение: {}", message);
        }
        println!("The channel is closed.");
    });

    // Отправка некоторых сообщений
    for i in 0..5 {
        sender.send(format!("Message {}", i)).await.unwrap();
        sleep(Duration::from_millis(500)).await;
    }

    // Закрываем отправитель
    drop(sender);

    // Ждем завершения задачи
    let _ = handle.await;
}
