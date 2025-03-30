#Sakura Pomodoro Timer with Spotify Integration

## Overview

The **Sakura Pomodoro Timer** is a simple, elegant Pomodoro timer built with Rust, featuring a clean, minimalist design inspired by the delicate beauty of cherry blossoms (Sakura). This timer enhances productivity by dividing work sessions into focused intervals separated by short breaks, during which users can enjoy their favorite Spotify playlist seamlessly.

## Features

- **Pomodoro Technique:** Default 25-minute work intervals.
- **Spotify Integration:** Automatically plays your chosen Spotify playlist during Pomodoro sessions.
- **Minimalist Sakura Design:** A visually pleasing and distraction-free interface.

## How It Works

- Click **"Start Pomodoro 🍅"** to initiate a 25-minute work session.
- Once the session starts, your selected Spotify playlist automatically begins playing.
- Monitor the countdown displayed clearly in minutes and seconds.
- Reset your session anytime by clicking the **"Reset"** button.

## Getting Started

### Prerequisites
- [Rust](https://rustup.rs/) installed.
- A [Spotify Developer](https://developer.spotify.com/dashboard/) account with a created Spotify app.

### Installation

Clone the repository and navigate into the project directory:

```bash
git clone https://github.com/your-repo/sakura_pomodoro.git
cd sakura_pomodoro
```

Replace `SPOTIFY_CLIENT_ID`, `SPOTIFY_CLIENT_SECRET`, and `YOUR_PLAYLIST_ID` in `src/main.rs` with your Spotify credentials.

Then run:

```bash
cargo run
```

## Technologies Used
- Rust
- Egui for the graphical interface
- rspotify for Spotify integration

## Contributing
Contributions, suggestions, and improvements are welcome!

## License
This project is open-sourced under the MIT license.
