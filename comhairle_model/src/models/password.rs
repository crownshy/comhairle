//! Password hashing and strength validation.
//!
//! Lives in the model layer because user creation (`models::users`) hashes and
//! validates passwords as part of building the row.

use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use rand_core::OsRng;

use crate::models::error::AuthError;

/// Validate password strength according to security requirements
///
/// Requirements:
/// - Minimum 16 characters
/// - Must include characters from at least 3 of 4 categories:
///   - Uppercase letters (A-Z)
///   - Lowercase letters (a-z)
///   - Numbers (0-9)
///   - Special characters
/// - Uses zxcvbn for additional complexity checking
pub fn validate_password_strength(password: &str) -> Result<(), AuthError> {
    let mut errors = Vec::new();

    // Check minimum length
    if password.len() < 16 {
        errors.push(format!(
            "Password must be at least 16 characters long (current length: {})",
            password.len()
        ));
    }

    // Check character categories
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password
        .chars()
        .any(|c| !c.is_alphanumeric() && !c.is_whitespace());

    let category_count = [has_uppercase, has_lowercase, has_digit, has_special]
        .iter()
        .filter(|&&x| x)
        .count();

    if category_count < 3 {
        let mut missing_categories = Vec::new();
        if !has_uppercase {
            missing_categories.push("uppercase letters (A-Z)");
        }
        if !has_lowercase {
            missing_categories.push("lowercase letters (a-z)");
        }
        if !has_digit {
            missing_categories.push("numbers (0-9)");
        }
        if !has_special {
            missing_categories.push("special characters (e.g., !@#$%^&*)");
        }

        errors.push(format!(
            "Password must include characters from at least 3 of 4 categories. Consider adding: {}",
            missing_categories.join(", ")
        ));
    }

    // Use zxcvbn for additional complexity checking
    let entropy = zxcvbn::zxcvbn(password, &[]);

    // zxcvbn scores range from 0 (weak) to 4 (strong)
    // We require a score of at least 3
    use zxcvbn::Score;
    if matches!(entropy.score(), Score::Zero | Score::One | Score::Two) {
        let feedback = entropy.feedback();
        let warning = feedback
            .and_then(|f| f.warning())
            .map(|w| format!("{:?}", w))
            .unwrap_or_else(|| "Password is too predictable or common".to_string());

        let suggestions = feedback.and_then(|f| {
            let suggs = f.suggestions();
            if suggs.is_empty() {
                None
            } else {
                Some(
                    suggs
                        .iter()
                        .map(|s| format!("{:?}", s))
                        .collect::<Vec<String>>()
                        .join("; "),
                )
            }
        });

        let mut complexity_msg = format!("Password complexity is too low. {}", warning);
        if let Some(suggs) = suggestions {
            complexity_msg.push_str(&format!(" Suggestions: {}", suggs));
        }
        errors.push(complexity_msg);
    }

    if !errors.is_empty() {
        return Err(AuthError::WeakPassword(errors.join(". ")));
    }

    Ok(())
}

/// Generate a hashed password
pub fn hash_pw(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AuthError::PasswordHash)?;

    Ok(hash.to_string())
}
