use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    console_debug!("Received request: {req:?}");
    Response::ok("Hello, World!")
}
