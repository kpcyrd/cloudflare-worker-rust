use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    console_debug!("Received request: {req:?}");
    Response::ok("Hello, World!")
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
