use aws_sdk_backup::{Client, Error};

pub async fn list_backup_jobs() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let resp = client.list_backup_jobs().send().await?;

    if let Some(jobs) = resp.backup_jobs {
        for job in jobs {
            println!("{:?}", job);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_list_backup_jobs() {
        let result = list_backup_jobs().await;
        assert!(result.is_ok());
    }

}


