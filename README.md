
[![Members](https://img.shields.io/badge/dynamic/json?style=for-the-badge&label=&logo=discord&logoColor=white&labelColor=black&color=%23f3f3f3&query=$.approximate_member_count&url=https%3A%2F%2Fdiscord.com%2Fapi%2Finvites%2FENB7RbxVZE%3Fwith_counts%3Dtrue)](https://discord.gg/ENB7RbxVZE)&nbsp;[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge&logo=5865F2&logoColor=black&labelColor=black&color=%23f3f3f3)](https://github.com/AndrewShedov/enter-text--LARS/blob/main/LICENSE)

# Enter Text (LARS)

This application is a prototype developed to prepare for the transition of [CRYSTAL](https://github.com/CrystalSystems) to a new technology stack — **LARS** (Leptos, Actix Web, Rust, ScyllaDB).<br/>
This prototype allows you to **add**, **display**, **update**, and **delete text**, demonstrating a full **CRUD** operation cycle with the ScyllaDB database.

<p align="center">
  <img src="https://raw.githubusercontent.com/AndrewShedov/enter-text--LARS/refs/heads/main/assets/gif.gif"/>
</p>

**Composition:** <br/>
[Full code](https://github.com/AndrewShedov/enter-text--LARS/tree/main/main) | [Cargo.toml](https://github.com/AndrewShedov/enter-text--LARS/blob/main/main/Cargo.toml)<br/>

**Structure:** <br/>
**L**eptos v0.8.2<br/>
**A**ctix Web v4.x<br/>
**R**ust v1.92.0<br/>
**S**cyllaDB v2025.4.0<br/>

**Local PC Specifications:** <br/>
OS: Debian 12<br/>
 
### Key Features: 

1. **Auto-Schema, Single-Row Architecture, and Constant-id.**<br>
Upon application startup, the <code>data</code> table is automatically created (if it does not exist) to store the entered text. During the save operation, a row is formed in the table, consisting of three columns: <code>id</code> (Primary Key), <code>content</code> (text data), and <code>created_at</code> (timestamp). A constant <code>id</code> of <code>UUID format (11111111-1111-1111-1111-111111111111)</code> is used for the entered text. Instead of creating multiple entries, the system uses the <code>INSERT</code> operation as an <code>"upsert"</code> (updating an existing record). Since the <code>id</code> is always the same, any save operation simply overwrites the data in the content column for this specific row.

<p align="center">
  <img src="https://raw.githubusercontent.com/AndrewShedov/enter-text--LARS/refs/heads/main/assets/screenshot_1.png"/>
</p>
<p align="center"><strong>Screenshot 1: Single Row View</strong></p>


2. **Blocking SSR (SsrMode::PartiallyBlocked)**: A blocking server-side rendering mode. This ensures that dynamic content from ScyllaDB (and all other site text) is "injected" into the HTML structure directly on the server. As a result, search engine crawlers receive a fully rendered document, ensuring 100% indexing and high SEO performance.

<p align="center">
  <img src="https://raw.githubusercontent.com/AndrewShedov/enter-text--LARS/refs/heads/main/assets/screenshot_2.png"/>
</p>
<p align="center"><strong>Screenshot 2: Server-Side Rendered (SSR) Source Code View (Ctrl+U)</strong></p>


3. **Fine-grained Reactivity**: When text is updated, only the specific DOM node containing that text is re-rendered, while the rest of the page remains untouched. This behavior is achieved through Leptos reactive signals.

4. **Isomorphic Data Access & Reactive UI**: The use of <code>Resource::new_blocking</code> ensures seamless state synchronization between the server and the client. It automatically monitors database changes via action versions, allowing the UI to instantly toggle buttons (e.g., switching between "Add" and "Update" or showing the "Delete" button) without a page reload.

5. **Asynchronous ScyllaDB Integration**: High-performance asynchronous connection via <code>scylla-rust-driver</code>. Using a shared <code>Arc<Session></code> allows the server to handle thousands of concurrent requests simultaneously without blocking CPU threads while waiting for database responses. This ensures maximum system performance under heavy load.

6. **SSR Isolation**: All database interaction code is protected by <code>#[cfg(feature = "ssr")]</code> macros. This guarantees that database drivers and sensitive logic never leave the server.

7. **Informative Server Logging**: The system outputs clear and visual operation reports to the console, significantly simplifying monitoring and debugging of the server side:


<p align="center">
    <img src="https://raw.githubusercontent.com/AndrewShedov/enter-text--LARS/refs/heads/main/assets/screenshot_3.1.png" width="750" />
</p>
<p align="center"><strong>Screenshot 3: Server Startup and ScyllaDB Readiness Log</strong></p>

### Installation & Setup

**Compatibility Note:** This project is verified to work on Debian 12. Development on Windows is not recommended, as the installation process for Leptos and ScyllaDB on that system can cause critical errors. A Linux-based environment is required for correct operation.

**1. Environment Preparation (Debian 12 and similar).**<br>
To ensure the project builds correctly, basic compilation tools and security libraries must be installed on the system.

Installing system dependencies:

```bash
sudo apt update && sudo apt install build-essential pkg-config libssl-dev -y
```

**2. Installing Rust and Leptos Tools**

The project is compatible with the Stable version of Rust, which guarantees build predictability.


2.1. Install Rust (installs the current stable version):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2.2. Configure environment variables:

```bash
source $HOME/.cargo/env
```

2.3. Add WebAssembly support:

```bash
rustup target add wasm32-unknown-unknown
```

2.4. Install the cargo-leptos build tool:

```bash
cargo install --locked cargo-leptos
```

**3. ScyllaDB Installation and Configuration.**

Installation is performed directly on the Debian 12 system using the official ScyllaDB repository.

3.1. Update package indexes:

```bash
sudo apt-get update
```

3.2. Install necessary system utilities:

```bash
sudo apt-get install -y apt-transport-https curl gnupg
```

3.3. Add the official ScyllaDB repository to the system:

```bash
curl -sSf https://get.scylladb.com/server | sudo bash
```

3.4. Interactive configuration and ScyllaDB installation:

```bash
sudo scylla_setup
```

3.5. Start the ScyllaDB server service:

```bash
sudo systemctl start scylla-server
```

3.6. Check the status of cluster nodes:

```bash
nodetool status
```

### Database Preparation

Before the first project launch, you must create a Keyspace in ScyllaDB.

Enter the database console using the:

```bash
cqlsh
```

and execute the query:

```bash
CREATE KEYSPACE IF NOT EXISTS prototype 
WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1};
```

This **keyspace** is required for the application to function correctly.

### Launching the project

1. Cloning the project repository:

```bash
git clone https://github.com/AndrewShedov/enter-text--LARS && cd enter-text--LARS/main 
```

2. Launch the project:

```bash
cargo leptos watch
```

Once the build is complete, the application will be available at:

<code>http://127.0.0.1:3000</code>

The **data** table inside the **prototype**  keyspace will be created automatically upon the application's first request to the database, enabled by the built-in Auto-Schema logic.<br>

By default, ScyllaDB is configured to work with the address <code>127.0.0.1:9042</code>.<br>
You can verify the address by entering the command into the terminal:

```bash
cqlsh
```

After entering the command, the address should be displayed:<br>

<code>Connected to at 127.0.0.1:9042</code>
<br>
<br>

[![SHEDOV.TOP](https://img.shields.io/badge/SHEDOV.TOP-black?style=for-the-badge)](https://shedov.top/) 
[![CRYSTAL](https://img.shields.io/badge/CRYSTAL-black?style=for-the-badge)](https://crystal.you/AndrewShedov)
[![Discord](https://img.shields.io/badge/Discord-black?style=for-the-badge&logo=discord&color=black&logoColor=white)](https://discord.gg/ENB7RbxVZE)
[![Telegram](https://img.shields.io/badge/Telegram-black?style=for-the-badge&logo=telegram&color=black&logoColor=white)](https://t.me/ShedovTop)
[![X](https://img.shields.io/badge/%20-black?style=for-the-badge&logo=x&logoColor=white)](https://x.com/AndrewShedov)
[![VK](https://img.shields.io/badge/VK-black?style=for-the-badge&logo=vk)](https://vk.com/ShedovTop)
[![VK Video](https://img.shields.io/badge/VK%20Video-black?style=for-the-badge&logo=vk)](https://vkvideo.ru/@ShedovTop)
[![YouTube](https://img.shields.io/badge/YouTube-black?style=for-the-badge&logo=youtube)](https://www.youtube.com/@AndrewShedov)