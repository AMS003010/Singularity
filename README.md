```
01010011 01101001 01101110 01100111 01110101 01101100 01100001 01110010 01101001 01110100 01111001


███████╗██╗███╗   ██╗ ██████╗ ██╗   ██╗██╗      █████╗ ██████╗ ██╗████████╗██╗   ██╗
██╔════╝██║████╗  ██║██╔════╝ ██║   ██║██║     ██╔══██╗██╔══██╗██║╚══██╔══╝╚██╗ ██╔╝
███████╗██║██╔██╗ ██║██║  ███╗██║   ██║██║     ███████║██████╔╝██║   ██║    ╚████╔╝ 
╚════██║██║██║╚██╗██║██║   ██║██║   ██║██║     ██╔══██║██╔══██╗██║   ██║     ╚██╔╝  
███████║██║██║ ╚████║╚██████╔╝╚██████╔╝███████╗██║  ██║██║  ██║██║   ██║      ██║   
╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝  ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝   ╚═╝      ╚═╝


01010011 01101001 01101110 01100111 01110101 01101100 01100001 01110010 01101001 01110100 01111001 
```

( PS: It's a customizable dashboard powered by a templating engine with data injection having parallelism to speed up rendering 😅. 
With a Shared Cache to speed it up further⚡. All built in rust 🦀)

<br/>

# Singularity
Have all your stuff in one place, that's right 😌 all in your ___Singularity___.

![image](https://github.com/user-attachments/assets/39ea2b89-981e-43f6-b0f8-017bff644a9d)


<br/>

## Contents
* <a href="https://github.com/AMS003010/Singularity/blob/main/docs/install.md">Install</a>
* <a href="https://github.com/AMS003010/Singularity/blob/main/docs/configuration.md">Configuration</a>
* <a href="https://github.com/AMS003010/Singularity/blob/main/docs/preconfigured-pages.md">Preconfigured pages</a>
* <a href="https://github.com/AMS003010/Singularity/blob/main/docs/themes.md">Themes</a>


<br/>

## Features
### Various widgets
* Weather 🌤️
* Clock ⏰
* Calendar 🗓️
* Header
* _Will be adding more in the near (distant) future_

### Cache
A shared cache is present which is enabled by default with a TTL (Time To Live) of 5 minutes. So all the widgets with cache support will be cached for a duration of 5 minutes after which it will be refetched.

```
theme: neo-noir
theme_background_color: "black"
widget_heading: "white"
footer: "yellow"
cache: 5m
```

The `TTL` can be changed by changing the `cache` attribute in `singularity.yaml`. Supported formats
* `10m` : Here `TTL` is set to 10 minutes
* `2h` : Here `TTL` is set to 2 hours
* `0.5h` : Here `TTL` is set to 30 minutes
* `0.5m` : Here `TTL` is set to 30 seconds

<br/>

> [!NOTE]
> Have a minimum `TTL` of 10 seconds. If it is `< 10 seconds` then it will use the system default of `5m`.

<br/>

### Header Widget
It is a special monitoring widget which can be enabled with `header-widget: true` for each page. It is by default set to `false` and is positioned by default after the the navbar in each page. It can be set specifically for each page. 

It shows the `mounts` available in the system along with the disk space available in `GB` for each drive. It also shows the system stats like
* CPU Usage 🖥
* No. of Cores 🧇
* Wi-Fi status 🛜
* OS 💽
* Username 🖥️

<br/>

## Install

### Windows 🪟

#### Prerequisites (Need to install manually)
- rustc
- Cargo
- rustup

Run this command to build the executable and run it
```
git clone https://github.com/AMS003010/Singularity.git
cd Singularity
cargo r -r
```

<br/>

### Linux 🐧
_This was tested on a Ubuntu 24.04 LTS_

Run these commands. This script installs everything for you including the prerequisites. `install-linux.sh` will install the prerequisites, build the exectable and add it to path (cargo's path).
```
git clone https://github.com/AMS003010/Singularity.git
cd Singularity/scripts/
chmod +x install-linux.sh
mv install-linux.sh ../
cd ..
./install-linux.sh
```

Now open a new terminal under the root of the singularity directory and run the executable, type
```
singularity
```

<br/>

### Run it as container 📦

#### Prerequisites
- docker

```
git clone https://github.com/AMS003010/Singularity.git
cd Singularity
docker compose up
```

<br/>

## Activity

![Alt](https://repobeats.axiom.co/api/embed/cdf8cb589bbd9eacf7b0f133ba4744847e64e98c.svg "Repobeats analytics image")

<br/>

## Profiling
```
cargo install samply --locked
cargo build && samply record target/release/singularity
```

<br/>

> [!CAUTION]
> The project is under active development, expect things to break every once in a while 😑.

> [!NOTE]
> Inspired from [Glance](https://github.com/glanceapp/glance)
