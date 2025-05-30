use base64::encode;
use captcha::{gen, Difficulty};
use rustgram_server_util::error::{ServerCoreError, ServerErrorConstructor};
use rustgram_server_util::get_time;
use rustgram_server_util::res::AppRes;
use sentc_crypto_common::AppId;

use crate::customer::customer_model;
use crate::customer::customer_model::CaptchaEntity;
use crate::ApiErrorCodes;

pub async fn captcha(app_id: impl Into<AppId>) -> AppRes<(String, String)>
{
	let (solution, png) = create_captcha()?;
	let id = customer_model::save_captcha_solution(app_id, solution).await?;

	Ok((id, png))
}

pub async fn validate_captcha(app_id: impl Into<AppId>, captcha_id: String, solution: String) -> AppRes<()>
{
	let app_id = app_id.into();

	let captcha = match CaptchaEntity::get_captcha_solution(&captcha_id, &app_id).await? {
		Some(c) => c,
		None => {
			return Err(ServerCoreError::new_msg(
				400,
				ApiErrorCodes::CaptchaNotFound,
				"Captcha not found",
			))
		},
	};

	//captcha is 20 min valid
	if captcha.time + (1000 * 20 * 60) < get_time()? {
		customer_model::delete_captcha(&app_id, captcha_id).await?;

		return Err(ServerCoreError::new_msg(
			400,
			ApiErrorCodes::CaptchaTooOld,
			"Captcha is too old, please do the captcha again",
		));
	}

	if captcha.solution != solution {
		customer_model::delete_captcha(app_id, captcha_id).await?;

		return Err(ServerCoreError::new_msg(
			400,
			ApiErrorCodes::CaptchaWrong,
			"Captcha is wrong",
		));
	}

	Ok(())
}

fn create_captcha() -> AppRes<(String, String)>
{
	let (solution, png) = gen(Difficulty::Easy)
		.as_tuple()
		.ok_or_else(|| ServerCoreError::new_msg(400, ApiErrorCodes::CaptchaCreate, "Can't create a captcha"))?;

	let png = encode(png);

	Ok((solution, png))
}
