# 🎯 Rust OG Fetcher

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Rocket](https://img.shields.io/badge/Rocket-FF4A00?style=for-the-badge&logo=rocket&logoColor=white)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)
![Made with Love](https://img.shields.io/badge/Made%20with-❤-red.svg?style=for-the-badge)

---

Hey! 👋 This is my little experiment with Rust - a simple Open Graph metadata fetcher. 
I'm learning Rust and thought it would be fun to build something useful!

[Quick Start](#-quick-start) • [API Usage](#-api-usage) • [Contributing](#-contributing)

</div>

## 🤔 What's This About?

I've been coding for a few years and wanted to try out Rust (because everyone keeps talking about how amazing it is! 🦀). 
This project is my attempt at building something practical while learning - it fetches Open Graph metadata from websites.
Is it perfect? Probably not! But it works, and I learned a ton building it! 😄

## ✨ What It Does

* ⚡️ Fetches metadata from websites (and it's pretty quick, thanks Rust!)
* 🎯 Grabs stuff like titles, descriptions, and those preview images you see on social media
* 🛠 Simple API that's easy to use (because complex APIs are no fun)
* 🔑 Basic API key setup (nothing fancy, but hey, it works!)
* 🤓 Lots of comments in the code so you can learn from my journey

## 🚀 Want to Try It?

### You'll Need

* Rust (latest stable should work)
* Cargo (comes with Rust)
* A curious mind! 🧠

### Setup

1. Grab the code:
```bash
git clone https://github.com/yourusername/rust-og-fetcher.git
cd rust-og-fetcher
```

2. Set up your secret key:
```bash
echo "API_KEY=whatever_you_want_here" > .env
```

3. Fire it up:
```bash
cargo run
```

## 🎮 How to Use It

### The Basic Stuff

Just send a GET request to:
```
http://localhost:8000/fetch_meta?url=<website-you-want-to-check>
```

Don't forget to include your API key in the headers:
```bash
x-api-key: whatever_you_put_in_env
```

### Example

Try this in your terminal:
```bash
curl -H "x-api-key: your_key" "http://localhost:8000/fetch_meta?url=https://example.com"
```

You'll get something like this:
```json
{
    "title": "Example Website",
    "description": "This is what the website is about",
    "og_image": "https://example.com/cool-image.jpg"
}
```

## 🔧 What I Used

* **Rocket**: Makes web stuff in Rust less scary
* **reqwest**: For fetching websites (like fetch/axios but in Rust)
* **scraper**: Helps grab the metadata from HTML
* **serde**: Handles all the JSON stuff
* **dotenv**: For that API key configuration

## 🤝 Want to Help?

Found a bug? Have a cool idea? Want to teach me better Rust practices? I'm all ears! 

1. Fork it
2. Create your branch (`git checkout -b feature/CoolFeature`)
3. Commit your changes (`git commit -m 'Added something cool!'`)
4. Push to the branch (`git push origin feature/CoolFeature`)
5. Open a Pull Request and let's chat!

## 📝 License

MIT License - do whatever you want with the code! Just don't blame me if something breaks 😅

## ⭐ Like What You See?

Drop a star if this helped you learn something new about Rust or if you found it useful!

---

<div align="center">

Made with ❤️ while learning 🦀 Rust

PS: If you're also learning Rust, let's connect! I'm always up for learning from others.

</div>