use crate::domain::vendor::{Vendor, VendorAddress, VendorLocation};
use crate::domain::vendor_error::VendorError;
use crate::ports::vendor_source::VendorSource;
use cenyslovensko_web_client::WebClient;
use derive_more::Constructor;
use serde::Deserialize;

#[derive(Debug, Clone, Constructor)]
pub struct HttpVendorSource {
    web_client: WebClient,
    vendor_path: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetVendorResponse {
    branch_id: String,
    branch_name: String,
    address: GetVendorAddressResponse,
    company_id: String,
    location: GetVendorLocationResponse,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetVendorAddressResponse {
    city: String,
    street_number: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetVendorLocationResponse {
    lat: f64,
    lng: f64,
}

impl From<GetVendorResponse> for Vendor {
    fn from(response: GetVendorResponse) -> Self {
        Vendor::new(
            response.branch_id,
            response.branch_name,
            VendorAddress::new(response.address.city, response.address.street_number),
            response.company_id,
            VendorLocation::new(response.location.lat, response.location.lng),
        )
    }
}

impl VendorSource for HttpVendorSource {
    async fn get_vendor(&self) -> Result<Vec<Vendor>, VendorError> {
        let endpoint = self
            .web_client
            .resolve_url(&self.vendor_path)
            .map_err(|error| VendorError::Unavailable(error.to_string()))?;

        let response = self
            .web_client
            .client()
            .get(endpoint)
            .send()
            .await
            .map_err(|error| VendorError::Unavailable(error.to_string()))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(VendorError::NotFound);
        }

        let response = response
            .error_for_status()
            .map_err(|error| VendorError::Unavailable(error.to_string()))?;

        let response: Vec<GetVendorResponse> = response
            .json()
            .await
            .map_err(|error| VendorError::InvalidResponse(error.to_string()))?;

        Ok(response.into_iter().map(Vendor::from).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cenyslovensko_web_client::WebClientConfig;
    use httpmock::Method::GET;
    use httpmock::MockServer;

    #[tokio::test]
    async fn returns_vendor_from_valid_json_response() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/vendor");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    // language=json
                    r#"[
                    {
                        "branchId":"branch_1",
                        "branchName":"Main Branch",
                        "address":{"city":"Bratislava","streetNumber":"123"},
                        "companyId":"company_1",
                        "location":{"lat":48.8566,"lng":2.3522}
                    }
                    ]"#,
                );
        });

        let web_client = WebClientConfig::new(server.base_url())
            .build()
            .expect("web client should build");
        let source = HttpVendorSource::new(web_client, "vendor".to_string());

        let result = source.get_vendor().await;

        assert_eq!(
            result,
            Ok(vec![Vendor::new(
                "branch_1".into(),
                "Main Branch".into(),
                VendorAddress::new("Bratislava".into(), "123".into()),
                "company_1".into(),
                VendorLocation::new(48.8566, 2.3522),
            )])
        );
    }

    #[tokio::test]
    async fn returns_not_found_for_404_response() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/vendor");
            then.status(404);
        });

        let web_client = WebClientConfig::new(server.base_url())
            .build()
            .expect("web client should build");
        let source = HttpVendorSource::new(web_client, "vendor".to_string());

        let result = source.get_vendor().await;

        assert_eq!(result, Err(VendorError::NotFound));
    }

    #[tokio::test]
    async fn returns_invalid_response_for_non_json_body() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/vendor");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{"missing_vendor":"branch_1"}"#);
        });

        let web_client = WebClientConfig::new(server.base_url())
            .build()
            .expect("web client should build");
        let source = HttpVendorSource::new(web_client, "vendor".to_string());

        let result = source.get_vendor().await;

        assert!(matches!(result, Err(VendorError::InvalidResponse(_))));
    }

    #[tokio::test]
    async fn returns_unavailable_for_server_errors() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/vendor");
            then.status(500);
        });

        let web_client = WebClientConfig::new(server.base_url())
            .build()
            .expect("web client should build");
        let source = HttpVendorSource::new(web_client, "vendor".to_string());

        let result = source.get_vendor().await;

        assert!(matches!(result, Err(VendorError::Unavailable(_))));
    }
}
