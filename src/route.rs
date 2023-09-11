use crate::user;

use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/hi/{greet}", web::get().to(user::hi));
  // config.route("/login", web::post().to(user::login));

  // config.service(
  //   web::scope("/user")
  //   .route("/session", web::get().to(user::user_session))
  //   .route("/get/{user_id}", web::get().to(user::get_user))
  //   .route("/update/{user_id}", web::post().to(user::update_user))
  //   .route("/delete/{user_id}", web::post().to(user::delete_user))
  // );

}