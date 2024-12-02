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

#### Various widgets
* Weather
* Clock
* Calendar

<br/>
(PS: Under the hood, it's basically a templating engine with data injection having parallelism to speed up rendering 😅)
<br/>

#### Behind the scenes ⚠️⚠️⚠️
![image](https://github.com/user-attachments/assets/6f6bd473-2425-4208-b681-9c2515ed3ce8)
