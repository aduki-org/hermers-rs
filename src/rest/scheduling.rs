//! Scheduling / booking resource.

use serde_json::{json, Value};

use crate::rest::error::HermesError;
use crate::rest::http::{list_query, Client};
use crate::rest::types::{
    Appointment, AppointmentData, Availability, Override, Page, Query, Service, Window,
};

/// Booking services and appointments.
pub struct Scheduling {
    http: Client,
}

impl Scheduling {
    pub(crate) fn new(http: Client) -> Self {
        Self { http }
    }

    /// Public service view by slug.
    pub async fn view(&self, slug: &str) -> Result<Service, HermesError> {
        self.http.get(&format!("/book/{slug}"), None).await
    }

    /// Public book.
    pub async fn book(
        &self,
        slug: &str,
        name: &str,
        email: &str,
        start: &str,
        end: &str,
    ) -> Result<Value, HermesError> {
        self.http
            .post(
                &format!("/book/{slug}"),
                &json!({
                    "name": name,
                    "email": email,
                    "start": start,
                    "end": end,
                }),
                None,
            )
            .await
    }

    /// Guest view by token.
    pub async fn guest(&self, token: &str) -> Result<Value, HermesError> {
        self.http.get(&format!("/book/guest/{token}"), None).await
    }

    /// Cancel as guest.
    pub async fn cancel_guest(&self, token: &str) -> Result<Value, HermesError> {
        self.http
            .post(&format!("/book/guest/{token}/cancel"), &json!({}), None)
            .await
    }

    /// Create appointment.
    pub async fn create_appointment(
        &self,
        data: &AppointmentData,
    ) -> Result<Appointment, HermesError> {
        self.http.post("/user/appointments", data, None).await
    }

    /// List appointments.
    pub async fn appointments(
        &self,
        query: Option<Query>,
    ) -> Result<Page<Appointment>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/appointments", opts.as_ref()).await
    }

    /// Active appointments.
    pub async fn active_appointments(
        &self,
        query: Option<Query>,
    ) -> Result<Page<Appointment>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get("/user/appointments/active", opts.as_ref())
            .await
    }

    /// Retrieve appointment.
    pub async fn retrieve_appointment(&self, hex: &str) -> Result<Appointment, HermesError> {
        self.http
            .get(&format!("/user/appointments/{hex}"), None)
            .await
    }

    /// Guests for an appointment.
    pub async fn guests(&self, hex: &str) -> Result<Vec<crate::rest::types::Guest>, HermesError> {
        self.http
            .get(&format!("/user/appointments/{hex}/guests"), None)
            .await
    }

    /// Update appointment status.
    pub async fn update_appointment_status(
        &self,
        hex: &str,
        status: &str,
    ) -> Result<Appointment, HermesError> {
        self.http
            .patch(
                &format!("/user/appointments/{hex}/status"),
                &json!({ "status": status }),
                None,
            )
            .await
    }

    /// Cancel appointment.
    pub async fn cancel_appointment(&self, hex: &str) -> Result<Value, HermesError> {
        self.http
            .patch(
                &format!("/user/appointments/{hex}/cancel"),
                &json!({}),
                None,
            )
            .await
    }

    /// Delete appointment.
    pub async fn delete_appointment(&self, hex: &str) -> Result<Value, HermesError> {
        self.http
            .delete(&format!("/user/appointments/{hex}"), None)
            .await
    }

    /// Create service.
    pub async fn create_service(&self, body: &Value) -> Result<Service, HermesError> {
        self.http.post("/user/services", body, None).await
    }

    /// List services.
    pub async fn services(&self) -> Result<Vec<Service>, HermesError> {
        self.http.get("/user/services", None).await
    }

    /// Retrieve service.
    pub async fn retrieve_service(&self, hex: &str) -> Result<Service, HermesError> {
        self.http.get(&format!("/user/services/{hex}"), None).await
    }

    /// Delete service.
    pub async fn delete_service(&self, hex: &str) -> Result<Value, HermesError> {
        self.http
            .delete(&format!("/user/services/{hex}"), None)
            .await
    }

    /// Windows.
    pub async fn windows(&self) -> Result<Vec<Window>, HermesError> {
        self.http.get("/user/windows", None).await
    }

    /// Overrides.
    pub async fn overrides(&self) -> Result<Vec<Override>, HermesError> {
        self.http.get("/user/overrides", None).await
    }

    /// Availability.
    pub async fn availability(&self, start: &str, end: &str) -> Result<Availability, HermesError> {
        self.http
            .get(&format!("/user/availability/{start}/{end}"), None)
            .await
    }
}
