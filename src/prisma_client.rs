use crate::prisma::new_client;
use crate::prisma::PrismaClient;

use once_cell::sync::OnceCell;

static CLIENT: OnceCell<PrismaClient> = OnceCell::new();
static PRISMA_INITIALIZED: OnceCell<tokio::sync::Mutex<bool>> = OnceCell::new();

pub async fn get_prisma() -> Option<&'static PrismaClient> {
    let client_option = CLIENT.get();
    if let Some(_) = client_option {
        return client_option;
    }

    let initializing_mutex = PRISMA_INITIALIZED.get_or_init(|| tokio::sync::Mutex::new(false));

    let mut initialized = initializing_mutex.lock().await;
    
    if !*initialized {
        match new_client().await {
            Ok(client) => if let Ok(_) = CLIENT.set(client) {
                *initialized = true;
            },
            Err(error) => panic!("Error occurred whilst connecting to client: {:?}", error),
        } 
    }
    drop(initialized);
    CLIENT.get()
}