# Singularity
Have all your stuff in one place, that's right 😌 all in your ___Singularity___ 

(PS: Under the hood, it's basically a templating engine with data injection having parallelism to speed up rendering 😅)
<br/>

## Contents
* <a href="https://github.com/AMS003010/Singularity/blob/main/docs/install.md">Install</a>
* <a href="https://github.com/AMS003010/Singularity/blob/main/docs/configuration.md">Configuration</a>
* <a href="https://github.com/AMS003010/Singularity/blob/main/docs/preconfigured-pages.md">Preconfigured pages</a>
* <a href="https://github.com/AMS003010/Singularity/blob/main/docs/themes.md">Themes</a>

#### Behind the scenes ⚠️⚠️⚠️
![image](https://github.com/user-attachments/assets/6f6bd473-2425-4208-b681-9c2515ed3ce8)



### Features
#### Various widgets
* Weather
* Clock
* Calendar

## Install

### Windows

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

### Linux
This was tested on a Ubuntu 24.04 LTS

This script installs everything for you including the prerequisites 
```
git clone https://github.com/AMS003010/Singularity.git
cd Singularity/scripts/install-linux.sh
chmod +x install-linux.sh
mv install-linux.sh ../
./install-linux.sh
```

Now to run the executable, type
```
singularity
```

### Run it as container

```
docker compose up
```

> [!CAUTION]
>
> The project is under active development, expect things to break every once in a while 😑.

<br/>

> [!NOTE]
> Inspired from [Glance](https://github.com/glanceapp/glance)
