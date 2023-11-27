use warp::Filter;
use prometheus::{register_int_counter, Opts, Encoder, IntGaugeVec};
use log::{error, info};

#[tokio::main]
async fn main() {
  env_logger::init();

  let request_counter = register_int_counter!(
    Opts::new("request_counter", "Number of webhooks requested")
  )
  .unwrap();

    // Custom metric for distance
  let distance_metric = IntGaugeVec::new(
      Opts::new("distance_metric", "Distance metric from webhooks"),
      &["source"],
  )
  .expect("Failed to create distance metric");

  // Define a filter for the /webhook endpoint
  let webhook = warp::path!("webhook")
    .and(warp::post())
    .and(warp::header::optional::<String>("X-Forwarded-For"))
    .and(warp::header::optional::<String>("Sender-Name"))
    .and(warp::body::json())
    .map(move|_forwarded_for: Option<String>, _sender_name: Option<String>, payload: serde_json::Value| {

      request_counter.inc();

      info!("Webhook received: {:?}", payload);

   // Extract and update the distance metric
      if let Some(metrics) = payload.get("metrics").and_then(|metrics| metrics.as_object()) {
        if let Some(distance_value) = metrics.get("distance").and_then(|value| value.as_i64()) {
          distance_metric.with_label_values(&["webhook"]).set(distance_value);
        }
      }

      // Perform actions based on the webhook payload
      // std::fs::write("webhook.json", serde_json::to_string_pretty(&payload).unwrap()).expect("Unable to write file");
      // Respond to the webhook request
      warp::reply::html("Webhook received")
  });
  
  let metrics = warp::path!("metrics")
    .map(move|| {
      let mut buffer = vec![];
      if let Err(err) = prometheus::TextEncoder::new().encode(&prometheus::gather(), &mut buffer) {
          error!("Failed to encode Prometheus metrics: {}", err);
      }
      warp::reply::with_header(buffer, "content-type", "text/plain")
    });
  
  let routes = warp::any().and(webhook.or(metrics));
  // Start the warp server
  warp::serve(routes)
    .run(([192,168,31,92], 5000))
    .await;
}
