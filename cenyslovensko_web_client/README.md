# CenySlovensko Web Client

Shared Rust web client for CenySlovensko modules. This crate centralizes HTTP client configuration such as base URI,
shared headers, timeouts, proxy, and log level.

> [!NOTE]
> This client is intended to be reused by API modules (for example `cenyslovensko_version_api`) to keep transport
> configuration consistent.

> [!IMPORTANT]
> This project is not affiliated with or endorsed by CenySlovensko. It is an independent implementation.
