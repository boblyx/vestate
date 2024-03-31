# Overview
- A Simple real estate info navigator.
- Display plans, cost / sqm
- 3D viewer (switchable to orthogonal) to view layout to review:
    - Natural Lighting (glare / sun exposure)
    - Natural Ventilation (low resolution airflow using LBM)
    - Views
    - Nearby buildings
- Strengths / Weaknesses list:
    - Views
    - Proximity to amenities
    - Available Facilities
    - Maintenance Fees
    - Pests
    - Neighbours

# Planned Tech Stack
- UI: [Dioxus](https://github.com/dioxuslabs/dioxus) + [tailwind](https://github.com/tailwindlabs/tailwindcss)
- 3D:  bevy-dioxus (?)
- API: [actix-web](https://github.com/actix/actix-web)
- DB: [MongoDB](https://github.com/mongodb/mongo)

# Development

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

And in a separate terminal
```bash
bash tailwindcss.sh
```

- Open the browser to http://localhost:8080

# TODO
- [x] Basic Setup for CSS and multiple components
- [ ] Basic setup for bevy-dioxus to render 3D
- [ ] Landing Page
- [ ] Schema for estate item and units, strengths and weaknesses
- [ ] Panel for adding / removing entries

# Motivation
- Learn Rust
- Make a somewhat useful app
