## Preconfigured Pages

Find some preconfigured config files
(Paste them to your singularity.yaml)

### Sample 1

![image](https://github.com/user-attachments/assets/39ea2b89-981e-43f6-b0f8-017bff644a9d)
```
theme: neo-noir
theme_background_color: "black"
widget_heading: "white"
footer: "yellow"
cache: 5m
pages:
  - name: Home
    header-widget: true
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
  - name: Juice
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
  - name: Bun
    header-widget: true
    columns:
      - size: small
        widgets:
          - type: weather
            location: London, United Kingdom
  - name: Stuff
    header-widget: true
    columns:
      - size: small
        widgets:
          - type: calendar
  - name: Crème-Brûlée
    header-widget: true
    columns:
      - size: small
        widgets:
          - type: clock
```
