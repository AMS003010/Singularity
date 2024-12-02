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
./install-linux.sh
cd ..
```

Now to run the executable, type
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

> [!CAUTION]
> The project is under active development, expect things to break every once in a while 😑.
