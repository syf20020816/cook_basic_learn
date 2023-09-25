fn block_task() -> () {
    println!("block task...")
}

async fn async_task(number: u8) -> u8 {
    number
}

#[tokio::main]
async fn main() {
    let mut blocks = Vec::new();
    for _x in 0..9 {
        //生成同步任务
        blocks.push(tokio::task::spawn_blocking(block_task))
    }
    //生成异步任务
    let async_task_tokio = tokio::spawn(async_task(1));
    let number = async_task_tokio.await.unwrap();
    println!("{}", number);
    //遍历同步任务的Vector
    for block in blocks {
        let _ = block.await;
    }

}