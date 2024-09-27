use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SeedPhraseSync {
    seed_phrase: String,
}

impl SeedPhraseSync {
    pub fn new(seed_phrase: &str) -> Self {
        Self {
            seed_phrase: seed_phrase.to_string(),
        }
    }

    pub fn sync_to_device(&self) -> Result<(), anyhow::Error> {
        // Logic to sync across devices using the seed phrase
        Ok(())
    }
}
