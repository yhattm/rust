mod aws;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    println!("The sum of 5 and 6 is {}", rust::add(5, 6));
    let result = aws::backup::list_backup_jobs().await;
    println!("Result: {:?}", result);
}
