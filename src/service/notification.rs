use std::thread;

use bambangshop::{Result, compose_error_response};
use rocket::http::Status;
use crate::model::notification::Notification;
use crate::model::product::Product;
use crate::model::subscriber::Subscriber;
use crate::repository::subscriber::SubscriberRepository;

pub struct NotificationService;

impl NotificationService {
    pub fn subscriber(product_type: &str, subscriber: Subscriber) -> Result<Subscriber> {
        let product_type_upper: String = product_type.to_uppercase();
        let product_type_str: &str = product_type_upper.as_str();
        let subscriber_result: Subscriber = SubscriberRepository::add(product_type_str, subscriber);
        return Ok(subscriber_result);
    }

    pub fn subscribe(product_type: &str) -> Result<SubscriberRequest> {
    let product_type_clone = String::from(product_type);
    return thread::spawn(move || Self::subscribe_request(product_type_clone))
        .join()
        .unwrap();
    }

    #[tokio::main]
    async fn subscribe_request(product_type: String) -> Result<SubscriberRequest> {
        let product_type_upper: String = product_type.to_uppercase();
        let product_type_str: &str = product_type_upper.as_str();
        let notification_receiver_url: String = format!("{}/receive", APP_CONFIG.get_instance_root_url());
        let payload: SubscriberRequest = SubscriberRequest {
            name: APP_CONFIG.get_instance_name().to_string(),
            url: notification_receiver_url
        };

        let request_url: String = format!("{}/notification/subscribe/{}", APP_CONFIG.get_publisher_root_url(), product_type_str);
        let request = REQUEST_CLIENT
            .post(request_url.clone())
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(to_string(&payload).unwrap())
            .send()
            .await;
        log::warn!("Sent subscribe request to: {}", request_url);

        return match request {
            Ok(f) => match f.json::<SubscriberRequest>().await {
                Ok(x) => Ok(x),
                Err(y) => Err(compose_error_response(
                    Status::NotAcceptable,
                    y.to_string()
                ))
            },
            Err(e) => Err(compose_error_response(
                Status::NotFound,
                e.to_string()
            ))
        };
    }
}

pub fn unsubscribe(product_type: &str, url: &str) -> Result<Subscriber> {
    let product_type_upper: String = product_type.to_uppercase();
    let product_type_str: &str = product_type_upper.as_str();
    let result: Option<Subscriber> = SubscriberRepository::delete(product_type_str, url);
    if result.is_none() {
        return Err(compose_error_response(
            Status::NotFound,
            String::from("Subscriber not found.")
        ));
    }
    return Ok(result.unwrap());
}
