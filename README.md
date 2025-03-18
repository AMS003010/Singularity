![Singularity](https://github.com/user-attachments/assets/2ce71654-284e-4cfd-ac25-044290ea25d2)

# Singularity
> Have all your stuff in one place. That's right 😌 — all in your **Singularity**.

![Dashboard Preview](https://github.com/user-attachments/assets/39ea2b89-981e-43f6-b0f8-017bff644a9d)

Singularity is a **customizable dashboard** powered by a **templating engine with data injection** and built-in **parallelism** for faster rendering 😅. It features a **shared cache** for even greater speed ⚡ — all crafted in **Rust** 🦀.

---

## 📌 Table of Contents
- [📥 Installation](#-installation)
- [⚙️ Configuration](https://github.com/AMS003010/Singularity/blob/main/docs/configuration.md)
- [📑 Preconfigured Pages](https://github.com/AMS003010/Singularity/blob/main/docs/preconfigured-pages.md)
- [🎨 Themes](https://github.com/AMS003010/Singularity/blob/main/docs/themes.md)
- [🔥 Features](#-features)
- [🐎 Profiling](#-profiling)

---

## 🔥 Features

### 📌 Widgets
- Weather 🌤️
- Clock ⏰
- Calendar 🗓️
- Header Widget
- _More widgets coming soon!_

### ⚡ Shared Cache
Singularity features a **shared cache** enabled by default, with a **Time-To-Live (TTL) of 5 minutes**. This improves performance by reducing redundant fetches.

#### 🛠️ Configuring TTL in `singularity.yaml`
```yaml
theme: neo-noir
theme_background_color: "black"
widget_heading: "white"
footer: "yellow"
cache: 5m
```

✅ Supported Formats:
- `10m` → 10 minutes
- `2h` → 2 hours
- `0.5h` → 30 minutes
- `0.5m` → 30 seconds

> **⚠️ NOTE:** Minimum TTL is **10 seconds**. If set below this, the system defaults to **5 minutes**.

### 📊 Header Widget
A special **monitoring widget** that displays system stats. Enable it for each page with:
```yaml
header-widget: true
```
#### 📡 Displays:
- **CPU Usage** 🖥
- **Number of Cores** 🧇
- **Wi-Fi Status** 🛜
- **OS Info** 💽
- **Username** 🖥️
- **Available Mounts & Disk Space**

![System Stats](https://github.com/user-attachments/assets/b74282ed-fa32-4781-98d1-dbe9dc94e716)

---

## 📥 Installation

### 🪟 Windows
#### Prerequisites
- `rustc`
- `Cargo`
- `rustup`

#### Install & Run
```sh
git clone https://github.com/AMS003010/Singularity.git
cd Singularity
cargo r -r
```

### 🐧 Linux _(Tested on Ubuntu 24.04 LTS)_
#### Install & Run
```sh
git clone https://github.com/AMS003010/Singularity.git
cd Singularity/scripts/
chmod +x install-linux.sh
mv install-linux.sh ../
cd ..
./install-linux.sh
```
Then, run:
```sh
singularity
```

### 📦 Run as a Container
#### Prerequisites
- `Docker`

#### Install & Run
```sh
git clone https://github.com/AMS003010/Singularity.git
cd Singularity
docker compose up
```

---

## 📊 Activity
![Repobeats](https://repobeats.axiom.co/api/embed/cdf8cb589bbd9eacf7b0f133ba4744847e64e98c.svg)

---

## 🐎 Profiling
Optimize performance with `samply`:
```sh
cargo install samply --locked
cargo build && samply record target/release/singularity
```

---

## ⚠️ Caution
> **This project is under active development.** Expect occasional breakages 😑.

### 🌟 Inspired by [Glance](https://github.com/glanceapp/glance)

---

👨‍💻 Maintainers
This project is maintained by Abhijith M S (AMS003010).

🤝 Contributing
We welcome contributions to Twilight! To contribute:

Fork the repository.
Create a new branch (feature-branch).
Commit your changes.
Push to your branch and open a pull request.
📝 License
This project is licensed under the MIT License.

📩 Contact For any queries or issues, feel free to reach out via GitHub Issues.

Happy Coding! 🚀
