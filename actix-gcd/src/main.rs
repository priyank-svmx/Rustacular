use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    return m * n;
}
fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("computing the GCD with zero is boring.");
    }

    let response = format!(
        "the greatest common divisor of the numbers {} and {} is <b> {} </b>",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    return HttpResponse::Ok().content_type("text/html").body(response);
}

fn main() {
    println!("Hello, world!");

    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:3000..welcome welcome");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding sever to address")
        .run()
        .expect("error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
              <title> GCD calculator</title>
              <form action="/gcd"  method="post">
               <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit"> compute gcd</button>
                </form>
            "#,
    )
}
