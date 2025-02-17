use anyhow::{anyhow, Context};
use fastrand::Rng;
use tokio::task;

use argon2::password_hash::SaltString;
use argon2::{password_hash, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

/// 生成 hash
///
/// ## Arguments
///
/// - `password`: 用户输入的明文密码
pub async fn hash(password: String) -> anyhow::Result<String> {
    task::spawn_blocking(move || {
        // 生成16字节随机数据作为盐
        let mut salt_bytes = [0u8; 16];
        Rng::new().fill(&mut salt_bytes);
        let salt = SaltString::encode_b64(&salt_bytes)
            .map_err(|e| anyhow!(e).context("failed to generate salt"))?;

        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!(e).context("failed to hash password"))?
            .to_string())
    })
    .await
    .context("panic in hash()")?
}

/// 验证 hash
///
/// ## Arguments
///
/// - `password`: 用户输入的明文密码
/// - `hash`：数据库中保存的 hash
pub async fn verify(password: String, hash: String) -> anyhow::Result<bool> {
    task::spawn_blocking(move || {
        let hash = PasswordHash::new(&hash)
            .map_err(|e| anyhow!(e).context("BUG: password hash invalid"))?;

        let res = Argon2::default().verify_password(password.as_bytes(), &hash);

        match res {
            Ok(()) => Ok(true),
            Err(password_hash::Error::Password) => Ok(false),
            Err(e) => Err(anyhow!(e).context("failed to verify password")),
        }
    })
    .await
    .context("panic in verify()")?
}

#[cfg(test)]
mod tests {
    use super::{hash, verify};

    #[tokio::test]
    async fn test_hash_and_verify_success() {
        let password = "my_secure_password".to_string();

        // Generate a hash
        let hashed_password = hash(password.clone())
            .await
            .expect("Failed to hash the password");

        // Verify the correct password
        let is_valid = verify(password, hashed_password.clone())
            .await
            .expect("Failed to verify the password");
        assert!(
            is_valid,
            "Expected the password to be valid for the generated hash"
        );
    }

    #[tokio::test]
    async fn test_verify_incorrect_password() {
        let password = "my_secure_password".to_string();
        let incorrect_password = "wrong_password".to_string();

        // Generate a hash
        let hashed_password = hash(password.clone())
            .await
            .expect("Failed to hash the password");

        // Verify an incorrect password
        let is_valid = verify(incorrect_password, hashed_password.clone())
            .await
            .expect("Failed to verify the password");
        assert!(
            !is_valid,
            "Expected the incorrect password to be invalid for the generated hash"
        );
    }

    #[tokio::test]
    async fn test_verify_invalid_hash_format() {
        let password = "my_secure_password".to_string();
        let invalid_hash = "invalid_hash_format".to_string();

        // Attempt to verify with an invalid hash format
        let result = verify(password, invalid_hash).await;

        assert!(
            result.is_err(),
            "Expected an error when verifying with an invalid hash format"
        );
        if let Err(e) = result {
            assert!(
                e.to_string().contains("password hash invalid"),
                "Unexpected error message: {}",
                e
            );
        }
    }

    #[tokio::test]
    async fn test_hash_panic_handling() {
        // Test the hashing function with a large password to induce potential failure
        let large_password = "a".repeat(1_000_000);

        let result = hash(large_password).await;

        assert!(
            result.is_ok(),
            "Expected no error when hashing an extremely large password"
        );
    }
}
