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
Working on caching up stuff, in order to speed it up further⚡. All built in rust 🦀)

<br/>

# Singularity
Have all your stuff in one place, that's right 😌 all in your ___Singularity___.

<br/>

## Contents
* <a href="https://github.com/AMS003010/Singularity/blob/main/docs/install.md">Install</a>
* <a href="https://github.com/AMS003010/Singularity/blob/main/docs/configuration.md">Configuration</a>
* <a href="https://github.com/AMS003010/Singularity/blob/main/docs/preconfigured-pages.md">Preconfigured pages</a>
* <a href="https://github.com/AMS003010/Singularity/blob/main/docs/themes.md">Themes</a>

<br/>

#### Behind the scenes ⚠️⚠️⚠️
![image](https://github.com/user-attachments/assets/d8d4732f-7adf-483c-aed3-241793e47179)


<br/>

## Features
### Various widgets
* Weather 🌤️
* Clock ⏰
* Calendar 🗓️
* _Will be adding more in the near (distant) future_

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
