use crate::session_state::TypedSession;
use crate::utils::{reject_anonymous_users, see_other};
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;

pub async fn log_out(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    let _user_id = reject_anonymous_users(&session).await?;

    session.log_out();
    FlashMessage::info("You have successfully logged out.").send();
    Ok(see_other("/login"))
}
