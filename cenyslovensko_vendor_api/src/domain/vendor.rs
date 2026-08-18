use derive_more::{Constructor, Display};
use serde::{Deserialize, Serialize};

/// Represents a vendor with branch information, address, company ID, and geographical location.
/// # Examples
/// ```
/// use cenyslovensko_vendor_api::domain::vendor::{Vendor, VendorAddress, VendorLocation};
/// let address = VendorAddress::new("Bratislava".to_string(), "123".
/// to_string());
/// let location = VendorLocation::new(48.8566, 2.3522
/// );
/// let vendor = Vendor::new("branch_1".to_string(), "Main Branch".to_string(), address, "company_1".to_string(), location);
/// assert_eq!(vendor.branch_id, "branch_1");
/// assert_eq!(vendor.branch_name, "Main Branch");
/// assert_eq!(vendor.address.city, "Bratislava");
/// assert_eq!(vendor.address.street_number, "123");
/// assert_eq!(vendor.company_id, "company_1");
/// assert_eq!(vendor.location.lat, 48.8566);
/// assert_eq!(vendor.location.lng, 2.3522);
/// ```
#[derive(Display, Debug, Clone, PartialEq, Constructor, Serialize)]
#[display(
    "Vendor(branch_id: {}, branch_name: {}, address: {}, company_id: {}, location: {})",
    branch_id,
    branch_name,
    address,
    company_id,
    location
)]
pub struct Vendor {
    pub branch_id: String,
    pub branch_name: String,
    pub address: VendorAddress,
    pub company_id: String,
    pub location: VendorLocation,
}

impl Eq for Vendor {}

/// Represents the address of a vendor with city and street number information.
/// # Examples
/// ```
/// use cenyslovensko_vendor_api::domain::vendor::VendorAddress;
///
/// let address = VendorAddress::new("Bratislava".to_string(), "123".
/// to_string());
/// assert_eq!(address.city, "Bratislava");
/// assert_eq!(address.street_number, "123");
/// ```
#[derive(Display, Debug, Clone, PartialEq, Constructor, Deserialize, Serialize)]
#[display("VendorAddress(city: {}, street_number: {})", city, street_number)]
pub struct VendorAddress {
    pub city: String,
    pub street_number: String,
}

impl Eq for VendorAddress {}

/// Represents the geographical location of a vendor with latitude and longitude coordinates.
/// # Examples
/// ```
/// use cenyslovensko_vendor_api::domain::vendor::VendorLocation;
///
/// let location = VendorLocation::new(48.8566, 2.3522);
/// assert_eq!(location.lat, 48.8566);
/// assert_eq!(location.lng, 2.3522);
/// ```
#[derive(Display, Debug, Clone, PartialEq, Constructor, Deserialize, Serialize)]
#[display("VendorLocation(lat: {}, lng: {})", lat, lng)]
pub struct VendorLocation {
    pub lat: f64,
    pub lng: f64,
}

impl Eq for VendorLocation {}
