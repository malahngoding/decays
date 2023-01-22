use bcrypt::{hash, verify};
use regex::Regex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, Validation};

use crate::{error::AppError, models::auth::Claims, KEYS};

// get 8 hours timestamp for jwt expiry
pub fn get_timestamp_8_hours_from_now() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let eighthoursfromnow = since_the_epoch + Duration::from_secs(28800);
    eighthoursfromnow.as_secs()
}
// verify token and extract data from it (a kind of middleware), whenever you try to extract claims in the handle it will first run this code
#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AppError::InvalidToken)?;
        let data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AppError::InvalidToken)?;
        Ok(data.claims)
    }
}

pub fn hash_password(s: &String) -> String {
    return hash(&s, 4).unwrap();
}

pub fn verify_password(hashed_str: &str, s: &str) -> bool {
    return verify(s, &hashed_str).unwrap();
}

pub fn is_valid_email(string: &str) -> bool {
    println!("{}", string);
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    return email_regex.is_match(string);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_timestamp_8_hours_from_now_test() {
        let second = get_timestamp_8_hours_from_now();
        assert_ne!(second, 1);
    }
    #[test]
    fn hash_password_test() {
        let password = "1234qwer!!".to_owned();
        let hashed_password = hash_password(&password);
        assert_ne!(password.len(), hashed_password.len());
    }
    #[test]
    fn verify_password_test() {
        let password = "1234qwer!!".to_owned();
        let hashed_password = hash_password(&password);
        let verified = verify_password(&hashed_password, &password);
        assert_eq!(verified, true);
    }
    #[test]
    fn is_valid_email_test() {
        let valid_email_addresses = [
            "foo@bar.com",
            "foo.bar42@c.com",
            "42@c.com",
            "f@42.co",
            "foo@4-2.team",
            "foo_bar@bar.com",
            "_bar@bar.com",
            "foo_@bar.com",
            "foo+bar@bar.com",
            "+bar@bar.com",
            "foo+@bar.com",
            "foo.lastname@bar.com",
        ];
        let invalid_email_addresses = ["foo at bar.com", ".x@c.com", "x.@c.com"];
        for email_address in &valid_email_addresses {
            assert_eq!(is_valid_email(email_address), true);
        }
        for email_address in &invalid_email_addresses {
            assert_eq!(is_valid_email(email_address), false);
        }
    }
}
