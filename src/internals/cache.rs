use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use dashmap::DashMap;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum CacheError {
    #[error("Failed to acquire cache lock")]
    LockError,
    #[error("Cache operation timed out")]
    TimeoutError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetCacheEntry {
    pub html: String,
    #[serde(with = "timestamp_serde")]
    pub timestamp: Instant,
}

// Custom serialization for Instant
mod timestamp_serde {
    use super::*;
    
    pub fn serialize<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = instant.duration_since(Instant::now());
        let system_time = SystemTime::now().checked_sub(duration)
            .unwrap_or(UNIX_EPOCH);
        
        system_time.duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .serialize(serializer)
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Instant, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(Instant::now() - Duration::from_secs(secs))
    }
}

pub struct GenericWidgetCache {
    cache: Mutex<DashMap<String, WidgetCacheEntry>>,
    ttl: Duration,
    lock_timeout: Duration,
}

impl GenericWidgetCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            cache: Mutex::new(DashMap::new()),
            ttl,
            lock_timeout: Duration::from_secs(5), // Default lock timeout
        }
    }

    // pub fn default() -> Self {
    //     Self::new(Duration::from_secs(3600))
    // }

    pub async fn get(&self, widget_name: &str) -> Result<Option<String>, CacheError> {
        let now = Instant::now();

        // Use a closure to limit the lock scope
        let cache_result = tokio::time::timeout(
            self.lock_timeout, 
            async {
                let cache = self.cache.lock().await;
                
                // Check if entry exists and is not expired
                cache.get(widget_name)
                    .filter(|entry| now.duration_since(entry.timestamp) < self.ttl)
                    .map(|entry| entry.html.clone())
            }
        )
        .await
        .map_err(|_| CacheError::TimeoutError)?;

        Ok(cache_result)
    }

    pub async fn insert(&self, widget_name: String, html: String) -> Result<(), CacheError> {
        let entry = WidgetCacheEntry {
            html,
            timestamp: Instant::now(),
        };

        tokio::time::timeout(
            self.lock_timeout, 
            async {
                let cache = self.cache.lock().await;
                cache.insert(widget_name, entry);
            }
        )
        .await
        .map_err(|_| CacheError::TimeoutError)?;

        Ok(())
    }

    // pub async fn clear(&self) -> Result<(), CacheError> {
    //     let now = Instant::now();

    //     tokio::time::timeout(
    //         self.lock_timeout, 
    //         async {
    //             let cache = self.cache.lock().await;
    //             cache.retain(|_, entry| now.duration_since(entry.timestamp) < self.ttl);
    //         }
    //     )
    //     .await
    //     .map_err(|_| CacheError::TimeoutError)?;

    //     Ok(())
    // }

    // pub async fn remove(&self, widget_name: &str) -> Result<(), CacheError> {
    //     tokio::time::timeout(
    //         self.lock_timeout, 
    //         async {
    //             let cache = self.cache.lock().await;
    //             cache.remove(widget_name);
    //         }
    //     )
    //     .await
    //     .map_err(|_| CacheError::TimeoutError)?;

    //     Ok(())
    // }

    // pub async fn len(&self) -> Result<usize, CacheError> {
    //     let result = tokio::time::timeout(
    //         self.lock_timeout, 
    //         async {
    //             let cache = self.cache.lock().await;
    //             cache.len()
    //         }
    //     )
    //     .await
    //     .map_err(|_| CacheError::TimeoutError)?;
    
    //     Ok(result)
    // }

    // Clone implementation
    pub fn clone(&self) -> Self {
        // Use blocking_lock to safely clone the cache contents
        let original_cache = self.cache.blocking_lock();
        
        // Create a new DashMap and manually copy entries
        let cloned_cache = DashMap::new();
        for entry in original_cache.iter() {
            cloned_cache.insert(entry.key().clone(), entry.value().clone());
        }
        
        Self {
            cache: Mutex::new(cloned_cache),
            ttl: self.ttl,
            lock_timeout: self.lock_timeout,
        }
    }
}

// Implement Clone trait
impl std::clone::Clone for GenericWidgetCache {
    fn clone(&self) -> Self {
        self.clone()
    }
}