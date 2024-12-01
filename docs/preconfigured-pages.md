## Preconfigured Pages

Find some preconfigured config files
(Paste them to your singularity.yaml)

### Sample 1

![image](https://github.com/user-attachments/assets/fee46fc2-f848-443a-8588-118b6f8be1a6)
```
theme: caffeine-rush
theme_background_color: "#F1E9D2"
widget_heading: "darkblue"
footer: "black"
pages:
  - name: Home
    columns:
      - size: small
        widgets:
          - type: clock
          - type: calendar
      - size: small
        widgets:
          - type: calendar
      - size: small
        widgets:
          - type: weather
            location: London, United Kingdom
          - type: calendar
```
