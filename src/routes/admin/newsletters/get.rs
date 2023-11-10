use actix_web::{http::header::ContentType, HttpResponse};

pub async fn get_newsletter_issue() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"<!DOCTYPE html>
        <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Change Password</title>
            </head>
            <body>
                <form action="/admin/newsletters" method="post">
                    <label>Title
                        <input
                            placeholder="Enter newsletter title"
                            name="title"
                        >
                    </label>
                    <br>
                    <label>Content Text
                        <input
                            placeholder="Enter content as plaintext"
                            name="text"
                        >
                    </label>
                    <br>
                    <label>Content Html
                        <input
                            placeholder="Enter content as html"
                            name="html"
                        >
                    </label>
                    <br>
                    <button type="submit">Send Newsletter</button>
                </form>
                <p><a href="/admin/dashboard">&lt;- Back</a></p>
            </body>
        </html>
        "#,
    ))
}
