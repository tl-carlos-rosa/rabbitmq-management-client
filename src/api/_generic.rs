use crate::errors::{RabbitMqApiError, RabbitMqClientError};
use http::StatusCode;
use reqwest::Response;
use serde::de::DeserializeOwned;

pub async fn handle_response<T>(response: Response) -> Result<T, RabbitMqClientError>
where
    T: DeserializeOwned,
{
    let status = response.status();

    if status.is_success() {
        return match response.json::<T>().await {
            Ok(response) => Ok(response),
            Err(e) => Err(RabbitMqClientError::ParsingError(e)),
        };
    }

    let text = response
        .text()
        .await
        .map_err(RabbitMqClientError::ParsingError)?;

    if status.eq(&StatusCode::UNAUTHORIZED) {
        return Err(RabbitMqClientError::Unauthorized);
    }

    if status.eq(&StatusCode::NOT_FOUND) {
        return Err(RabbitMqClientError::NotFound(text));
    }

    Err(RabbitMqClientError::ApiError(RabbitMqApiError {
        code: status,
        text,
    }))
}

pub async fn handle_empty_response(response: Response) -> Result<(), RabbitMqClientError> {
    let status = response.status();

    if status.is_success() {
        return Ok(());
    }

    if status.eq(&StatusCode::UNAUTHORIZED) {
        return Err(RabbitMqClientError::Unauthorized);
    }

    Err(RabbitMqClientError::ApiError(RabbitMqApiError {
        code: status,
        text: response
            .text()
            .await
            .map_err(RabbitMqClientError::ParsingError)?,
    }))
}
