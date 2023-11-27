use warp::Filter;

#[tokio::main]
async fn main() {
  // Define a filter for the /webhook endpoint
  let webhook = warp::path!("webhook")
  .and(warp::post())
  .and(warp::header::optional::<String>("X-Forwarded-For"))
  .and(warp::header::optional::<String>("Sender-Name"))
  .and(warp::body::json())
  .map(|_forwarded_for: Option<String>, _sender_name: Option<String>, payload: serde_json::Value| {

      println!("Webhook received: {:?}", payload);

      // Perform actions based on the webhook payload

      // Respond to the webhook request
      warp::reply::html("Webhook received")
  });

  // Start the warp server
  warp::serve(webhook)
    .run(([192,168,31,122], 3030))
    .await;
}
