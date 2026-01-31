use std::time::Duration;

use trpl;

fn main() {
    println!("[Test Thread]");
    trpl::block_on(test_thread());
    println!("[Test Future]");
    trpl::block_on(test_future());
    println!("[Test Future with messaging]");
    trpl::block_on(test_future_messaging());
    println!("[Test Future Timeout]");
    trpl::block_on(test_timeout());
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

async fn test_timeout() {
    let slow = async {
        trpl::sleep(Duration::from_secs(5)).await;
        "Finally finished"
    };

    match timeout(slow, Duration::from_secs(2)).await {
        Ok(message) => println!("Succeeded with '{message}'"),
        Err(duration) => {
            println!("Failed after {} seconds", duration.as_secs())
        }
    }
}

// This is a custom timeout function implemented by leveraging everything learned in the course so far.
// fn timeout<F: Future<Output = T>, T>(
//     f: F,
//     duration: Duration,
// ) -> impl Future<Output = Result<T, Duration>> {
//     async move {
//         let duration_job = async move {
//             trpl::sleep(duration).await;
//             duration
//         };

//         match trpl::select(f, duration_job).await {
//             trpl::Either::Left(result) => Ok(result),
//             trpl::Either::Right(duration) => Err(duration),
//         }
//     }
// }

// This function is based on the example from the book.
async fn timeout<F: Future>(f: F, duration: Duration) -> Result<F::Output, Duration> {
    match trpl::select(f, trpl::sleep(duration)).await {
        trpl::Either::Left(result) => Ok(result),
        trpl::Either::Right(_) => Err(duration),
    }
}
