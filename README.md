# 🔌 Laptop Guard

**Laptop Guard** is a security tool that alerts you via **Telegram** when your laptop is **unplugged** while running this application. This is especially useful when leaving your laptop charging in **public places** (libraries, coworking spaces, cafes) and wanting to be notified if someone tries to steal it.

> ⚠ **Important:** This tool does **not** prevent theft but provides an alert system for **real-time notifications** when your laptop is unplugged.

## 🔧 Features

- ✅ **Cross-platform:** Works on both **Windows** and **Linux**.
- 📡 **Telegram Notifications:** Sends an alert when your laptop is unplugged.
- 🔄 **Continuous Monitoring:** Runs in the background, checking the power state at regular intervals.
- 🔕 **Graceful Shutdown:** Stops running when you press **CTRL+C**.
- ⚡ **Configurable Check Interval:** Choose how frequently the system checks if the laptop is unplugged.
- 🤖 **Bot-Only Mode:** Allows running the Telegram bot alone to retrieve the required Chat ID.

## 🚩 Available Flags

| Flag                       | Description                                                                                   |
| -------------------------- | --------------------------------------------------------------------------------------------- |
| `-t, --token <TOKEN>`      | **(Required)** The **Telegram Bot API token**.                                                |
| `-c, --chat <CHAT_ID>`     | The **Telegram Chat ID** to send alerts to.                                                   |
| `-i, --interval <SECONDS>` | Sets the **interval** (in seconds) for checking the laptop’s power state (**Default:** `20`). |

## 🚀 How to Use

### **1️⃣ Download Laptop Guard**

Before running the application, you need to download the binary for your operating system.

#### **🐧 Download for Linux**

**Download** the Linux binary from the [latest release](https://github.com/DevotionLabs/laptop-guard/releases).

> Remember to give the file execution permission: `chmod +x laptop-guard-<VERSION>-linux`

#### **🪟 Download for Windows**

**Download** the Windows binary from the [latest release](https://github.com/DevotionLabs/laptop-guard/releases).

---

### **2️⃣ Create a Telegram Bot**

To receive alerts, you need to create a **Telegram bot** and get its **API token**:

1. Open **Telegram** and search for `@BotFather`.
2. Send the command `/newbot`
3. Follow the instructions and copy the **API token** provided.

---

### **3️⃣ Get Your Chat ID (First-Time Setup - Bot-Only Mode)**

Your **Chat ID** is required to send notifications. If you don’t know your Chat ID yet, run **Laptop Guard** in **Bot-Only Mode**:

#### **🐧 Running Bot-Only Mode on Linux**

```shell
./laptop-guard-<VERSION>-linux -t <YOUR_BOT_TOKEN>
```

#### **🪟 Running Bot-Only Mode on Windows**

```shell
./laptop-guard-<VERSION>-windows.exe -t <YOUR_BOT_TOKEN>
```

1. Start a chat with your newly created bot.
2. Send the `/chatid` command to your bot.
3. The bot will reply with your **Chat ID**.
4. Now you can **restart** the application with the **Chat ID** to enable laptop monitoring.

---

### **4️⃣ Run Laptop Guard in Full Mode (Monitoring & Alerts)**

Once you have your **Chat ID**, run Laptop Guard in **full mode**:

#### **🐧 Running on Linux**

```shell
./laptop-guard-<VERSION>-linux -t <YOUR_BOT_TOKEN> -c <YOUR_CHAT_ID> -i <INTERVAL_IN_SECONDS>
```

#### **🪟 Running on Windows**

```shell
./laptop-guard-<VERSION>-windows.exe -t <YOUR_BOT_TOKEN> -c <YOUR_CHAT_ID> -i <INTERVAL_IN_SECONDS>
```

---

## 🛠 Source

### 📦 Building from Source

Ensure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

git clone https://github.com/DevotionLabs/laptop-guard.git
cd laptop-guard
cargo build --release

The binary will be located in:

- **Linux:** `target/release/laptop_guard`
- **Windows:** `target/release/laptop_guard.exe`

### ▶️ Running from Source

After building, execute the binary:
cargo run -- -t <YOUR_BOT_TOKEN> -c <YOUR_CHAT_ID> -i 10
