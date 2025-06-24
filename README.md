# Zenth API Server

**Zenth API Server** is the central unified API layer of the Zenth Cloud ecosystem, developed by Sky Genesis Enterprise. It serves as the main integration gateway between all Zenth services, enabling modular communication, service orchestration, and user-level interaction via a secure, RESTful interface.

## 🔗 Features

- ✅ Unified API for all Zenth Cloud components
- ✅ RESTful and JSON-based interface
- ✅ OAuth2 / Token-based authentication (SSO-ready)
- ✅ Role-based access control (RBAC)
- ✅ Modular architecture for service discovery and registration
- ✅ Supports webhooks and event-driven workflows
- ✅ CLI-friendly and developer-first design
- ✅ Built-in metrics and monitoring endpoints

## 📦 Integrates With

Zenth API Server is at the heart of the Zenth Cloud stack, connecting:

- `ldap-server` – Identity & authentication provider
- `mail-server` – Mailbox and alias management
- `sip-server` – VoIP user provisioning
- `dns-server`, `dhcp-server` – Network config and zone control
- `panel-server` – Admin GUI powered by this API
- `status-server` – Status reporting and service health endpoints
- `firewall-server`, `vpn-server` – Access and device provisioning

## 🛠️ Technology

- Written in **Go** (Golang) for performance and simplicity
- OpenAPI/Swagger documentation for dev integration
- Pluggable modules for extensibility
- Rate limiting, audit logging, and request tracing included
- Containerized for use in Proxmox, Docker, or Kubernetes

## 📖 Documentation

API specs, authentication flow, usage examples and SDKs available in `/docs` or on [docs.zenthcloud.com](https://docs.zenthcloud.com).

## 🔐 Security

- All endpoints served over HTTPS
- Auth via tokens (JWT) or API keys
- Full audit logging and rate limiting per client/app

## 🛡️ License

This project is licensed under the **GNU Affero General Public License v3 (AGPLv3)**. Please see `LICENSE` for full terms.

---

Want to contribute or build on top of Zenth API Server? Start a PR, file an issue, or fork the project on [github.com/zenthcloud](https://github.com/zenthcloud).
