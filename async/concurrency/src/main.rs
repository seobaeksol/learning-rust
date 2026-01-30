use std::time::Duration;

use trpl;

fn main() {
    println!("[Test Thread]");
    trpl::block_on(test_thread());
    println!("[Test Future]");
    trpl::block_on(test_future());
    println!("[Test Future with messaging]");
    trpl::block_on(test_future_messaging());
}

async fn test_thread() {
    let handle = trpl::spawn_task(async {
        for i in 1..10 {
            println!("hi number {i} from the first task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the second task!");
        trpl::sleep(Duration::from_millis(500)).await;
    }

    handle.await.unwrap();
}

async fn test_future() {
    let fut1 = async {
        for i in 1..10 {
            println!("hi number {i} from the first task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let fut2 = async {
        for i in 1..10 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    trpl::join(fut1, fut2).await;
}

async fn test_future_messaging() {
    let (tx, mut rx) = trpl::channel();

    let tx1 = tx.clone();
    let tx_fut = async move {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let rx_fut = async {
        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    };

    let tx_fut2 = async move {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(1500)).await;
        }
    };

    trpl::join!(tx_fut, rx_fut, tx_fut2);
}
