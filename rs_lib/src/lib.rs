use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct KafkaProducer {
  producer: FutureProducer,
}

#[wasm_bindgen]
impl KafkaProducer {
  #[wasm_bindgen(constructor)]
  pub fn new(config: &str) -> Result<KafkaProducer, JsValue> {
    let producer: FutureProducer = ClientConfig::new()
      .set("bootstrap.servers", config)
      .create()
      .map_err(|e| {
        JsValue::from_str(&format!("Failed to create Kafka producer: {}", e))
      })?;

    Ok(KafkaProducer { producer })
  }

  pub async fn send(
    &self,
    topic: &str,
    key: Option<&str>,
    value: &str,
  ) -> Result<(), JsValue> {
    let record = FutureRecord::to(topic).key(key).payload(value);

    self.producer.send(record, 0).await.map_err(|e| {
      JsValue::from_str(&format!("Failed to send message to Kafka: {}", e))
    })?;

    Ok(())
  }
}
