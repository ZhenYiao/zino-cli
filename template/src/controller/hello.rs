use zino::{Request, Response, Result};

pub async fn index(req: Request) -> Result {
    let mut res = Response::default().context(&req);
    res.set_text_response("Hello World");
    Ok(res.into())
}
