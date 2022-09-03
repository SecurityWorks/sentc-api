/**
# Generated route files by rustgram route builder.

Please do not modify this file. Any changes will be overridden by the next route build.
Use the returned router instead
 */
use rustgram::{r, Router};

pub(crate) fn routes(router: &mut Router)
{
	router.post(
		"/api/v1/file",
		r(server_api::sentc_file_controller::register_file)
			.add(server_api::sentc_jwt_mw)
			.add(server_api::sentc_app_mw),
	);
	router.put(
		"/api/v1/file/:file_id",
		r(server_api::sentc_file_controller::update_file_name)
			.add(server_api::sentc_jwt_mw)
			.add(server_api::sentc_app_mw),
	);
	router.delete(
		"/api/v1/file/:file_id",
		r(server_api::sentc_file_controller::delete_file)
			.add(server_api::sentc_jwt_mw)
			.add(server_api::sentc_app_mw),
	);
	router.get(
		"/api/v1/file/:file_id",
		r(server_api::sentc_file_controller::get_file)
			.add(server_api::sentc_jwt_optional_mw)
			.add(server_api::sentc_app_mw),
	);
	router.get(
		"/api/v1/file/:file_id/part_fetch/:last_sequence",
		r(server_api::sentc_file_controller::get_parts).add(server_api::sentc_app_mw),
	);
	router.get(
		"/api/v1/file/part/:part_id",
		r(server_api::sentc_file_controller::download_part).add(server_api::sentc_app_mw),
	);
	router.post(
		"/api/v1/file/part/:session_id/:seq/:end",
		r(server_api::sentc_file_controller::upload_part)
			.add(server_api::sentc_jwt_mw)
			.add(server_api::sentc_app_mw),
	);
	router.post(
		"/api/v1/group/:group_id/file",
		r(server_api::sentc_file_controller::register_file_in_group)
			.add(server_api::sentc_group_mw)
			.add(server_api::sentc_jwt_mw)
			.add(server_api::sentc_app_mw),
	);
	router.get(
		"/api/v1/group/:group_id/file/:file_id",
		r(server_api::sentc_file_controller::get_file_in_group)
			.add(server_api::sentc_group_mw)
			.add(server_api::sentc_jwt_mw)
			.add(server_api::sentc_app_mw),
	);
	router.delete(
		"/api/v1/group/:group_id/file/:file_id",
		r(server_api::sentc_file_controller::delete_file_in_group)
			.add(server_api::sentc_group_mw)
			.add(server_api::sentc_jwt_mw)
			.add(server_api::sentc_app_mw),
	);
}
