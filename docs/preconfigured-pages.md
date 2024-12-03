## Preconfigured Pages

Find some preconfigured config files
(Paste them to your singularity.yaml)

### Sample 1

![image](https://github.com/user-attachments/assets/d8d4732f-7adf-483c-aed3-241793e47179)
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
