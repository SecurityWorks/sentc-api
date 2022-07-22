use std::error::Error;
use std::str::FromStr;

use jsonwebtoken::{decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use ring::rand;
use ring::signature::{self, KeyPair};
use sentc_crypto_common::UserId;
use serde::{Deserialize, Serialize};

use crate::core::api_err::{ApiErrorCodes, HttpErr};
use crate::core::get_time_in_sec;
use crate::customer::customer_entities::CustomerAppJwt;
use crate::user::user_entities::UserJwtEntity;

#[derive(Debug, Serialize, Deserialize)]
struct Claims
{
	//jwt defaults
	aud: String,
	sub: String,
	exp: usize,
	iat: usize,

	//sentc
	internal_user_id: UserId,
	user_identifier: String,
}

pub async fn create_jwt(
	internal_user_id: &str,
	user_identifier: &str,
	app_id: &str,
	customer_jwt_data: &CustomerAppJwt,
	aud: &str,
) -> Result<String, HttpErr>
{
	let iat = get_time_in_sec()?;
	let expiration = iat + 60 * 5; //exp in 5 min

	let claims = Claims {
		iat: iat as usize,
		aud: aud.to_string(),
		sub: app_id.to_string(),
		exp: expiration as usize,
		internal_user_id: internal_user_id.to_string(),
		user_identifier: user_identifier.to_string(),
	};

	let mut header = Header::new(Algorithm::from_str(customer_jwt_data.jwt_alg.as_str()).unwrap());
	header.kid = Some(customer_jwt_data.jwt_key_id.to_string());

	//TODO get it from the db (no cache for the sign key)
	let sign_key = "abc";
	//decode sign key
	let sign_key = base64::decode(sign_key).map_err(|_e| HttpErr::new(401, ApiErrorCodes::JwtCreation, "Can't create jwt", None))?;

	encode(&header, &claims, &EncodingKey::from_ec_der(&sign_key)).map_err(|e| {
		HttpErr::new(
			401,
			ApiErrorCodes::JwtCreation,
			"Can't create jwt",
			Some(format!("err in jwt creation: {}", e)),
		)
	})
}

pub async fn auth(jwt: &str, check_exp: bool) -> Result<(UserJwtEntity, usize), HttpErr>
{
	let header = decode_header(jwt).map_err(|_e| HttpErr::new(401, ApiErrorCodes::JwtWrongFormat, "Can't decode the jwt", None))?;
	let key_id = match header.kid {
		Some(k) => k,
		None => return Err(HttpErr::new(401, ApiErrorCodes::JwtWrongFormat, "Can't decode the jwt", None)),
	};
	let alg = header.alg;

	//TODO get the verify key from the db (no cache here because we would got extreme big cache for each app, and we may get the jwt from cache too)
	let verify_key = "abc";
	//decode the key
	let verify_key = base64::decode(verify_key).map_err(|_e| HttpErr::new(401, ApiErrorCodes::JwtWrongFormat, "Can't decode the jwt", None))?;

	let mut validation = Validation::new(alg);
	validation.validate_exp = check_exp;

	let decoded = decode::<Claims>(jwt, &DecodingKey::from_ec_der(&verify_key), &validation)
		.map_err(|_e| HttpErr::new(401, ApiErrorCodes::JwtValidation, "Wrong jwt", None))?;

	Ok((
		UserJwtEntity {
			id: decoded.claims.internal_user_id,
			identifier: decoded.claims.user_identifier,
			aud: decoded.claims.aud,
		},
		decoded.claims.exp,
	))
}

pub fn create_jwt_keys() -> Result<(String, String), HttpErr>
{
	let rng = rand::SystemRandom::new();
	let bytes = signature::EcdsaKeyPair::generate_pkcs8(&signature::ECDSA_P384_SHA384_FIXED_SIGNING, &rng).map_err(|e| map_create_key_err(e))?;

	let keypair =
		signature::EcdsaKeyPair::from_pkcs8(&signature::ECDSA_P384_SHA384_FIXED_SIGNING, bytes.as_ref()).map_err(|e| map_create_key_err(e))?;

	let verify_key = keypair.public_key();

	let verify_key = base64::encode(verify_key);
	let keypair = base64::encode(bytes);

	Ok((keypair, verify_key))
}

fn map_create_key_err<E: Error>(e: E) -> HttpErr
{
	HttpErr::new(
		500,
		ApiErrorCodes::JwtKeyCreation,
		"Can't create keys",
		Some(format!("Err in Jwt key creation: {}", e)),
	)
}