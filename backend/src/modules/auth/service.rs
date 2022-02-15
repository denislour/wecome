use super::dto::register::RegisterInput;
use super::dto::token::TokenClaim;
use crate::modules::user::dto::create::CreateUser;
use crate::modules::user::dto::update::UpdateUser;
use crate::modules::user::service::update_user_by_id;
use crate::modules::verification::dto::create::CreateVerification;
use crate::modules::verification::service::get_verification_by_id;
use crate::modules::{user, verification};
use crate::utils::email::send_verification_email;
use crate::UnwrappedPool;
use actix_web::error::ErrorBadRequest;
use actix_web::Error;
use branca::Branca;
use std::env;

pub fn verify_mail(db: &UnwrappedPool, verification_id: String) -> Result<(), Error> {
    let verification = match get_verification_by_id(db, verification_id) {
        Err(e) => return Err(ErrorBadRequest(e)),
        Ok(result) => result,
    };

    match update_user_by_id(
        db,
        verification.user_id,
        UpdateUser {
            is_active: Some(true),
            ..UpdateUser::default()
        },
    ) {
        Err(e) => Err(ErrorBadRequest(e)),
        Ok(_) => Ok(()),
    }
}

pub fn register(db: &UnwrappedPool, payload: RegisterInput) -> Result<String, Error> {
    let user = match user::service::create_user(
        db,
        CreateUser {
            nim: payload.nim,
            name: payload.name,
            password: payload.password,
            email: payload.email,
            ..CreateUser::default()
        },
    ) {
        Err(err) => return Err(ErrorBadRequest(err)),
        Ok(data) => data,
    };

    let verification = match verification::service::create_verification(
        db,
        CreateVerification {
            user_id: user.id.clone(),
            ..CreateVerification::default()
        },
    ) {
        Err(err) => return Err(ErrorBadRequest(err)),
        Ok(data) => data,
    };

    match send_verification_email(user.name, user.email, verification.clone().id) {
        Err(err) => return Err(ErrorBadRequest(err)),
        Ok(_) => {}
    };

    Ok(String::from("Successfully registered!"))
}

pub fn create_token(claim: TokenClaim) -> Result<String, branca::errors::Error> {
    let mut branca = Branca::new(&env::var("TOKEN_KEY").expect("token key not set").as_bytes())?;

    branca.encode(serde_json::to_string(&claim).unwrap().as_bytes())
}

pub fn get_token_data(token: String) -> Result<TokenClaim, branca::errors::Error> {
    let branca = Branca::new(&env::var("TOKEN_KEY").expect("token key not set").as_bytes())?;

    let decoded_token = branca.decode(&token, 0)?;

    let decoded_string = &String::from_utf8(decoded_token).expect("invalid UTF-8");

    let token_claim: TokenClaim = serde_json::from_str(decoded_string).unwrap();

    Ok(token_claim)
}
