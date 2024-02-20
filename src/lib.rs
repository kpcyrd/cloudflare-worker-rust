use worker::*;

/// Configure `wrangler.toml` to run this function periodically
///
/// Can be tested with `curl -v "http://localhost:8787/__scheduled?cron=*+*+*+*+*"`
/// However you need to use `wrangler dev --test-scheduled`
#[event(scheduled)]
async fn scheduled(event: ScheduledEvent, _env: Env, _ctx: ScheduleContext) {
    console_debug!("cronjob: {event:?}");
}

/// This function handles all requests
#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/", |req, _ctx| async move {
            let mut response = Response::from_html(format!(
                "<!DOCTYPE html>
<html>
    <head>
        <title>Hello, World! 🐱</title>
    </head>
    <body>
        <h1>🎷🐛</h1>
        <pre>{}</pre>
    </body>
</html>
",
                html_escape::encode_text(&format!("{req:#?}"))
            ))?;
            // https://github.com/cloudflare/workers-rs/pull/447
            response
                .headers_mut()
                .set("Content-Type", "text/html; charset=utf-8")?;
            Ok(response)
        })
        .post_async("/upload", file_upload)
        .run(req, env)
        .await
}

/// Request handler for `/upload`
async fn file_upload(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let Some(file) = req.form_data().await?.get("file") else {
        return Response::error("Bad Request", 400);
    };
    let FormEntry::File(buf) = file else {
        return Response::error("`file` part of POST form must be a file", 400);
    };

    Response::ok(format!("size = {}\n", buf.bytes().await?.len()))
}

#[cfg(test)]
mod tests {
    fn add(left: usize, right: usize) -> usize {
        left + right
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
