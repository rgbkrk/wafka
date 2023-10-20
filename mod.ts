import { instantiate } from "./lib/rs_lib.generated.js";

const { default: init, KafkaProducer } = await instantiate();

async function main() {
  await init();

  const producer = new KafkaProducer("localhost:9092");

  await producer.send("my-topic", null, "Hello, Kafka!");
}

main();
