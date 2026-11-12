# System Maintenance Daemon (Rust + D-Bus + Polkit)

This repository serves as a practical, complete reference implementation of a Linux system daemon written in **Rust**. It demonstrates how to securely bridge unprivileged client applications with a privileged root daemon using the **D-Bus System Bus** and **Polkit** (PolicyKit) authorization.

## 🎯 The Architecture

To understand how Linux manages privileges, we implemented a mock "System Maintenance" service containing:

*   **The State:** A single boolean (`is_active`) residing in the daemon's memory.
*   **The Daemon (`root`):** A Tokio-based async Rust program connected to the D-Bus System Bus via `zbus`.
*   **The Security (Polkit):** Write operations are protected by `zbus_polkit`. It prompts the user for an administrator password before proceeding and utilizes `auth_admin_keep` to retain authorization for subsequent rapid requests.
*   **The Client (`user`):** A lightweight Rust binary that generates a D-Bus proxy to communicate with the daemon natively, replacing the need for generic CLI tools like `busctl`.

## 🛠️ Prerequisites

*   Linux operating system with `dbus` and `polkit` installed.
*   Rust toolchain (edition 2024).

## 🚀 Setup & Installation

You must install the security policy files into your Linux system directories before running the code.

### 1. Install D-Bus Policy
The D-Bus policy allows our daemon to claim the well-known name `com.example.SystemMaintenance`.

```bash
sudo cp extras/com.example.SystemMaintenance.conf /etc/dbus-1/system.d/
```

### 2. Install Polkit Policy
The Polkit policy registers the `set-state` action and defines the authentication rules (configured with `auth_admin_keep` to avoid password fatigue).

```bash
sudo cp extras/com.example.systemmaintenance.policy /usr/share/polkit-1/actions/
```

## 💻 Running the Project

### Start the Daemon (Terminal 1)
The daemon must be run as root to comply with the D-Bus policy.

```bash
cargo build
sudo ./target/debug/system-maintenance-daemon
```
> **Expected Output:** `[Daemon] Successfully connected to the System Bus with Polkit security.`

### Run the Rust Client (Terminal 2)
Interact with the daemon as a standard user via the native Rust client.

```bash
cargo run --bin client
```
> **Expected Behavior:** Polkit will intercept the state change request and open a password prompt. Upon successful authentication, the state will toggle. Thanks to the `keep` policy, running the client again immediately after will bypass the password prompt.

## 📚 Libraries Used

*   [`tokio`](https://tokio.rs/): Asynchronous runtime.
*   [`zbus`](https://gitlab.freedesktop.org/dbus/zbus): Native Rust implementation of the D-Bus protocol.
*   [`zbus_polkit`](https://crates.io/crates/zbus_polkit): Polkit authorization integration for zbus.