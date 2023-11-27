use warp::Filter;
use prometheus::{register_int_counter, Opts, Encoder};

#[tokio::main]
async fn main() {
  let request_counter = register_int_counter!(
    Opts::new("request_counter", "Number of webhooks requested")
  )
  .unwrap();

  // Define a filter for the /webhook endpoint
  let webhook = warp::path!("webhook")
    .and(warp::post())
    .and(warp::header::optional::<String>("X-Forwarded-For"))
    .and(warp::header::optional::<String>("Sender-Name"))
    .and(warp::body::json())
    .map(move|_forwarded_for: Option<String>, _sender_name: Option<String>, payload: serde_json::Value| {

      request_counter.inc();

      println!("Webhook received: {:?}", payload);

      // Perform actions based on the webhook payload
      // std::fs::write("webhook.json", serde_json::to_string_pretty(&payload).unwrap()).expect("Unable to write file");
      // Respond to the webhook request
      warp::reply::html("Webhook received")
  });
  
  let metrics = warp::path!("metrics")
    .map(move|| {
      let mut buffer = vec![];
      prometheus::TextEncoder::new()
        .encode(&prometheus::gather(), &mut buffer).unwrap();
      warp::reply::with_header(buffer, "content-type", "text/plain")
    });
  
  let routes = warp::any().and(webhook.or(metrics));
  // Start the warp server
  warp::serve(routes)
    .run(([192,168,31,122], 5000))
    .await;
}
