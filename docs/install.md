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
