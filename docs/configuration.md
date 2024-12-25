## Configuration

You can control your dashboard with the `singularity.yaml`. 

<br/>

⚠️⚠️ Please have the `theme`, `theme_background_color`, `widget_heading`, `footer` and `page` fields in `singularity.yaml`.

<br/>

Control your body theme and footer theme
```
theme: neo-noir
theme_background_color: "black"
widget_heading: "white"
footer: "yellow"
```

Add a page named `Home`. We have a `clock` widget under column 1. We have a `calendar` widget under column 2. We have a `weather` widget under column 3. 
```
pages:
  - name: Home
    columns:
      - size: small
        widgets:
          - type: clock
      - size: small
        widgets:
          - type: calendar
      - size: small
        widgets:
          - type: weather
            location: London, United Kingdom
```

The above config will look like this in the below mockup
![image](https://github.com/user-attachments/assets/b83785cf-e37d-49c1-9ad1-b9dd39a4ae3a)

<br/>

## Features
### Widgets
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

![image](https://github.com/user-attachments/assets/b74282ed-fa32-4781-98d1-dbe9dc94e716)


It shows the `mounts` available in the system along with the disk space available in `GB` for each drive. It also shows the system stats like
* CPU Usage 🖥
* No. of Cores 🧇
* Wi-Fi status 🛜
* OS 💽
* Username 🖥️

<br/>

( PS: It's a customizable dashboard powered by a templating engine with data injection having parallelism to speed up rendering 😅. 
With a Shared Cache to speed it up further⚡. All built in rust 🦀)

<br/>

![image](https://github.com/user-attachments/assets/39ea2b89-981e-43f6-b0f8-017bff644a9d)
