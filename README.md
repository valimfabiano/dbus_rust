# System Maintenance Daemon (Rust + D-Bus + Polkit)

This repository serves as a practical, complete reference implementation of a Linux system daemon written in **Rust**. It demonstrates how to securely bridge unprivileged client applications with a privileged root daemon using the **D-Bus System Bus** and **Polkit** (PolicyKit) authorization.

## 🎯 The Architecture

To understand how Linux manages privileges (like changing network settings, mounting drives, or altering system power states), we implemented a mock "System Maintenance" service. 

*   **The State:** A single boolean (`is_active`) residing in the daemon's memory.
*   **The Daemon (`root`):** A Tokio-based async Rust program connected to the D-Bus System Bus via the `zbus` crate.
*   **The Security (Polkit):** Any user can read the state. However, if a user attempts to change the state, the daemon uses `zbus_polkit` to pause the request, query Polkit, and prompt the user for an administrator password before proceeding.

## 🛠️ Prerequisites

*   Linux operating system with `dbus` and `polkit` installed (standard on almost all distributions).
*   Rust toolchain (edition 2024).
*   `busctl` command-line utility (part of `systemd`).

## 🚀 Setup & Installation

Because this daemon connects to the **System Bus** and registers Polkit actions, you must install the security policy files into your Linux system directories before running the code.

### 1. Install D-Bus Policy
The D-Bus policy allows our daemon to claim the well-known name `com.example.SystemMaintenance`.

```bash
sudo cp extras/com.example.SystemMaintenance.conf /etc/dbus-1/system.d/
```

### 2. Install Polkit Policy
The Polkit policy registers the `com.example.systemmaintenance.set-state` action and defines the authentication rules.

```bash
sudo cp extras/com.example.systemmaintenance.policy /usr/share/polkit-1/actions/
```

## 💻 Running the Project

The daemon must be run as root to comply with the D-Bus policy.

### Start the Daemon (Terminal 1)

```bash
cargo build
sudo ./target/debug/system-maintenance-daemon
```
> **Expected Output:** `[Daemon] Successfully connected to the System Bus with Polkit security.`

### Interact as a Standard User (Terminal 2)

**1. Query the State (No password required)**

```bash
busctl --system call com.example.SystemMaintenance /com/example/SystemMaintenance com.example.SystemMaintenance State
```
> **Expected Output:** `b false`

**2. Change the State (Polkit Authentication Prompt)**

```bash
busctl --system call com.example.SystemMaintenance /com/example/SystemMaintenance com.example.SystemMaintenance SetState b true
```
> **Expected Behavior:** Polkit will intercept the call and open a password prompt (GUI or CLI). If you authenticate successfully as an admin, the state will change to `true`.

## 📚 Libraries Used

*   [`tokio`](https://tokio.rs/): Asynchronous runtime.
*   [`zbus`](https://gitlab.freedesktop.org/dbus/zbus): Native Rust implementation of the D-Bus protocol.
*   [`zbus_polkit`](https://crates.io/crates/zbus_polkit): Polkit authorization integration for zbus.