use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber {
    pub url: String,
    pub name: String,
}

impl Subscriber {
    #[tokio::main]
    pub async fn update(&self, payload: Notification) {
        REQUEST_CLIENT
            .post(&self.url)
            .header("Content-Type", "JSON")
            .body(to_string(&payload).unwrap())
            .send().await.ok();
        log::warn!("Sent {} notification of: [{}] {}, to: {}",
            payload.status, payload.product_type, payload.product_title, self.url);
    }
}

pub fn notify(&self, product_type: &str, status: &str, product: Product) {
    let mut payload: Notification = Notification {
        product_title: product.clone().title,
        product_type: String::from(product_type),
        product_url: product.clone().get_url(),
        subscriber_name: String::from(""),
        status: String::from(status)
    };

    let subscribers: Vec<Subscriber> = SubscriberRepository::list_all(product_type);
    for subscriber in subscribers {
        payload.subscriber_name = subscriber.clone().name;
        let subscriber_clone = subscriber.clone();
        let payload_clone = payload.clone();
        thread::spawn(move || subscriber_clone.update(payload_clone));
    }
}