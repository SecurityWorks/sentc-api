/**
# Generated route files by rustgram route builder.

Please do not modify this file. Any changes will be overridden by the next route build.
Use the returned router instead
 */
use rustgram::{r, Router};

use crate::middleware::*;

pub(crate) fn routes(router: &mut Router)
{
	router.post(
		"/api/v1/customer/register",
		r(crate::customer::register).add(app_token::app_token_base_app_transform),
	);
	router.post(
		"/api/v1/customer/prepare_login",
		r(crate::customer::prepare_login).add(app_token::app_token_base_app_transform),
	);
	router.post(
		"/api/v1/customer/done_login",
		r(crate::customer::done_login).add(app_token::app_token_base_app_transform),
	);
	router.get(
		"/api/v1/customer/captcha",
		r(crate::customer::customer_captcha).add(app_token::app_token_base_app_transform),
	);
	router.put(
		"/api/v1/customer/password_reset",
		r(crate::customer::prepare_reset_password).add(app_token::app_token_base_app_transform),
	);
	router.put(
		"/api/v1/customer/password_reset_validation",
		r(crate::customer::done_reset_password).add(app_token::app_token_base_app_transform),
	);
	router.put(
		"/api/v1/customer/refresh",
		r(crate::customer::refresh_jwt)
			.add(jwt::jwt_expire_transform)
			.add(app_token::app_token_base_app_transform),
	);
	router.post(
		"/api/v1/customer/register_validation",
		r(crate::customer::done_register)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_base_app_transform),
	);
	router.patch(
		"/api/v1/customer/email_resend",
		r(crate::customer::resend_email)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_base_app_transform),
	);
	router.put(
		"/api/v1/customer",
		r(crate::customer::update)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_base_app_transform),
	);
	router.put(
		"/api/v1/customer/data",
		r(crate::customer::update_data)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_base_app_transform),
	);
	router.put(
		"/api/v1/customer/password",
		r(crate::customer::change_password)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_base_app_transform),
	);
	router.delete(
		"/api/v1/customer",
		r(crate::customer::delete)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_base_app_transform),
	);
	router.get(
		"/api/v1/customer/apps/:last_fetched_time/:last_app_id",
		r(crate::customer_app::get_all_apps).add(jwt::jwt_transform),
	);
	router.post(
		"/api/v1/customer/app",
		r(crate::customer_app::create_app).add(jwt::jwt_transform),
	);
	router.get(
		"/api/v1/customer/app/:app_id",
		r(crate::customer_app::get_app_details).add(jwt::jwt_transform),
	);
	router.put(
		"/api/v1/customer/app/:app_id",
		r(crate::customer_app::update).add(jwt::jwt_transform),
	);
	router.put(
		"/api/v1/customer/app/:app_id/options",
		r(crate::customer_app::update_options).add(jwt::jwt_transform),
	);
	router.put(
		"/api/v1/customer/app/:app_id/file_options",
		r(crate::customer_app::update_file_options).add(jwt::jwt_transform),
	);
	router.delete(
		"/api/v1/customer/app/:app_id",
		r(crate::customer_app::delete).add(jwt::jwt_transform),
	);
	router.patch(
		"/api/v1/customer/app/:app_id/token_renew",
		r(crate::customer_app::renew_tokens).add(jwt::jwt_transform),
	);
	router.patch(
		"/api/v1/customer/app/:app_id/new_jwt_keys",
		r(crate::customer_app::add_jwt_keys).add(jwt::jwt_transform),
	);
	router.get(
		"/api/v1/customer/app/:app_id/jwt",
		r(crate::customer_app::get_jwt_details).add(jwt::jwt_transform),
	);
	router.delete(
		"/api/v1/customer/app/:app_id/jwt/:jwt_id",
		r(crate::customer_app::delete_jwt_keys).add(jwt::jwt_transform),
	);
	router.get(
		"/api/v1/user/:user_id/public_key",
		r(crate::user::get_public_key_data).add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/user/:user_id/public_key/:key_id",
		r(crate::user::get_public_key_by_id).add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/user/:user_id/verify_key/:key_id",
		r(crate::user::get_verify_key_by_id).add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/group/:group_id/public_key",
		r(crate::group::get_public_key_data).add(app_token::app_token_transform),
	);
	router.post(
		"/api/v1/exists",
		r(crate::user::exists).add(app_token::app_token_transform),
	);
	router.post(
		"/api/v1/register",
		r(crate::user::register).add(app_token::app_token_transform),
	);
	router.post(
		"/api/v1/prepare_login",
		r(crate::user::prepare_login).add(app_token::app_token_transform),
	);
	router.post(
		"/api/v1/done_login",
		r(crate::user::done_login).add(app_token::app_token_transform),
	);
	router.post(
		"/api/v1/user/prepare_register_device",
		r(crate::user::prepare_register_device).add(app_token::app_token_transform),
	);
	router.put(
		"/api/v1/refresh",
		r(crate::user::refresh_jwt)
			.add(jwt::jwt_expire_transform)
			.add(app_token::app_token_transform),
	);
	router.post(
		"/api/v1/init",
		r(crate::user::init_user)
			.add(jwt::jwt_expire_transform)
			.add(app_token::app_token_transform),
	);
	router.post(
		"/api/v1/keys/sym_key",
		r(crate::key_management::register_sym_key)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.delete(
		"/api/v1/keys/sym_key/:key_id",
		r(crate::key_management::delete_sym_key)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/keys/sym_key/master_key/:master_key_id/:last_fetched_time/:last_key_id",
		r(crate::key_management::get_all_sym_keys_to_master_key).add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/keys/sym_key/:key_id",
		r(crate::key_management::get_sym_key_by_id).add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/user/device/:last_fetched_time/:last_id",
		r(crate::user::get_devices)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.put(
		"/api/v1/user",
		r(crate::user::update)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.put(
		"/api/v1/user/done_register_device",
		r(crate::user::done_register_device)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.put(
		"/api/v1/user/update_pw",
		r(crate::user::change_password)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.put(
		"/api/v1/user/reset_pw",
		r(crate::user::reset_password)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.delete(
		"/api/v1/user",
		r(crate::user::delete)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.delete(
		"/api/v1/user/device/:device_id",
		r(crate::user::delete_device)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.post(
		"/api/v1/user/user_keys/rotation",
		r(crate::user::user_group_key_rotation)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/user/user_keys/rotation",
		r(crate::user::get_user_group_keys_for_update)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.put(
		"/api/v1/user/user_keys/rotation/:key_id",
		r(crate::user::done_key_rotation_for_device)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/user/user_keys/key/:key_id",
		r(crate::user::get_user_key)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/user/user_keys/keys/:last_fetched_time/:last_k_id",
		r(crate::user::get_user_keys)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.put(
		"/api/v1/user/user_keys/session/:key_session_id",
		r(crate::user::device_key_upload)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.post(
		"/api/v1/content",
		r(crate::content_management::create_non_related_content)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.post(
		"/api/v1/content/:user_id",
		r(crate::content_management::create_user_content)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.delete(
		"/api/v1/content/id/:content_id",
		r(crate::content_management::delete_content_by_id)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.delete(
		"/api/v1/content/item/:item",
		r(crate::content_management::delete_content_by_item)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/content/all/:last_fetched_time/:last_id",
		r(crate::content_management::get_content_all)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/content/:cat_id/:last_fetched_time/:last_id",
		r(crate::content_management::get_content_all_from_cat)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/content/user/all/:last_fetched_time/:last_id",
		r(crate::content_management::get_content_for_user)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/content/user/:cat_id/:last_fetched_time/:last_id",
		r(crate::content_management::get_content_for_user_from_cat)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/content/access/item/:item",
		r(crate::content_management::check_access_to_content_by_item)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.post(
		"/api/v1/content/group/:group_id",
		r(crate::content_management::create_group_content)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/content/group/:group_id/all/:last_fetched_time/:last_id",
		r(crate::content_management::get_content_for_group)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/content/group/:group_id/:cat_id/:last_fetched_time/:last_id",
		r(crate::content_management::get_content_for_group_from_cat)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.post(
		"/api/v1/search/group/:group_id",
		r(crate::content_searchable::create)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.delete(
		"/api/v1/search/group/:group_id",
		r(crate::content_searchable::delete)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/search/group/:group_id/all/:search/:last_fetched_time/:last_id",
		r(crate::content_searchable::search_all)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/search/group/:group_id/:cat_id/:search/:last_fetched_time/:last_id",
		r(crate::content_searchable::search_cat)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.post(
		"/api/v1/group",
		r(crate::group::create)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/group/all/:last_fetched_time/:last_group_id",
		r(crate::group::get_all_groups_for_user)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/group/invite/:last_fetched_time/:last_group_id",
		r(crate::group::get_invite_req)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/group/joins/:last_fetched_time/:last_group_id",
		r(crate::group::get_sent_join_req_for_user)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.delete(
		"/api/v1/group/joins/:join_req_id",
		r(crate::group::delete_sent_join_req_for_user)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.patch(
		"/api/v1/group/:group_id/invite",
		r(crate::group::accept_invite)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.delete(
		"/api/v1/group/:group_id/invite",
		r(crate::group::reject_invite)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.patch(
		"/api/v1/group/:group_id/join_req",
		r(crate::group::join_req)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/group/:group_id",
		r(crate::group::get_user_group_data)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/group/:group_id/light",
		r(crate::group::get_user_group_light_data)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/group/:group_id/update_check",
		r(crate::group::get_key_update_for_user)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/group/:group_id/keys/:last_fetched_time/:last_k_id",
		r(crate::group::get_user_group_keys)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/group/:group_id/key/:key_id",
		r(crate::group::get_user_group_key)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/group/:group_id/member/:last_fetched_time/:last_user_id",
		r(crate::group::get_group_member)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.post(
		"/api/v1/group/:group_id/child",
		r(crate::group::create_child_group)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.post(
		"/api/v1/group/:group_id/connected",
		r(crate::group::create_connected_group_from_group)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.delete(
		"/api/v1/group/:group_id",
		r(crate::group::delete)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.delete(
		"/api/v1/group/:group_id/leave",
		r(crate::group::leave_group)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.delete(
		"/api/v1/group/:group_id/kick/:user_id",
		r(crate::group::kick_user_from_group)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/group/:group_id/children/:last_fetched_time/:last_id",
		r(crate::group::get_all_first_level_children)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/group/:group_id/all/:last_fetched_time/:last_group_id",
		r(crate::group::get_all_groups_for_group)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/group/:group_id/invite/:last_fetched_time/:last_group_id",
		r(crate::group::get_invite_req_for_group)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/group/:group_id/joins/:last_fetched_time/:last_group_id",
		r(crate::group::get_sent_join_req_for_group)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.delete(
		"/api/v1/group/:group_id/joins/:join_req_id",
		r(crate::group::delete_sent_join_req_for_group)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.delete(
		"/api/v1/group/:group_id/invite/:group_id_to_reject",
		r(crate::group::reject_invite_group)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.patch(
		"/api/v1/group/:group_id/invite/:group_id_to_join",
		r(crate::group::accept_invite_group)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.put(
		"/api/v1/group/:group_id/invite/:invited_user",
		r(crate::group::invite_request)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.put(
		"/api/v1/group/:group_id/invite_group/:invited_group",
		r(crate::group::invite_request_to_group)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.put(
		"/api/v1/group/:group_id/invite_auto/:invited_user",
		r(crate::group::invite_auto)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.put(
		"/api/v1/group/:group_id/invite_group_auto/:invited_group",
		r(crate::group::invite_auto_group)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.put(
		"/api/v1/group/:group_id/invite/session/:key_session_id",
		r(crate::group::insert_user_keys_via_session_invite)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.put(
		"/api/v1/group/:group_id/change_rank",
		r(crate::group::change_rank)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.patch(
		"/api/v1/group/:group_id/change_invite",
		r(crate::group::stop_invite)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.patch(
		"/api/v1/group/:group_id/join_req/:group_id_to_join",
		r(crate::group::join_req_as_group)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/group/:group_id/join_req/:last_fetched_time/:last_user_id",
		r(crate::group::get_join_req)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.put(
		"/api/v1/group/:group_id/join_req/:join_user",
		r(crate::group::accept_join_req)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.delete(
		"/api/v1/group/:group_id/join_req/:join_user",
		r(crate::group::reject_join_req)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.put(
		"/api/v1/group/:group_id/join_req/session/:key_session_id",
		r(crate::group::insert_user_keys_via_session_join_req)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.post(
		"/api/v1/group/:group_id/key_rotation",
		r(crate::group::start_key_rotation)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.get(
		"/api/v1/group/:group_id/key_rotation",
		r(crate::group::get_keys_for_update)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
	router.put(
		"/api/v1/group/:group_id/key_rotation/:key_id",
		r(crate::group::done_key_rotation_for_user)
			.add(group::group_transform)
			.add(jwt::jwt_transform)
			.add(app_token::app_token_transform),
	);
}
