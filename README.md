# Lorem Ipsum Generator App

![alt text](preview.png)

A simple Lorem Ipsum Generator desktop application, built with Tauri, Sveltekit + Skeleton UI 

![Tauri Badge](https://img.shields.io/badge/Tauri-24C8D8?logo=tauri&logoColor=fff&style=for-the-badge)
![Rust Badge](https://img.shields.io/badge/Rust-000?logo=rust&logoColor=fff&style=for-the-badge)
![TypeScript Badge](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=fff&style=for-the-badge)
![Svelte Badge](https://img.shields.io/badge/Svelte-FF3E00?logo=svelte&logoColor=fff&style=for-the-badge)
![Tailwind CSS Badge](https://img.shields.io/badge/Tailwind%20CSS-06B6D4?logo=tailwindcss&logoColor=fff&style=for-the-badge)
![Bun Badge](https://img.shields.io/badge/Bun-000?logo=bun&logoColor=fff&style=for-the-badge)

## Installation

1. Clone repository

```
git clone https://github.com/liyunze-coding/lorem-ipsum-app.git
```

2. Either:

Install with Bun

```
bun install
```

OR

Delete `bun.lockb`, then install with:
```
npm install
pnpm install
deno install
```

Build it into a desktop application by doing:

```py
bun run tauri build # Or npm, pnpm, deno
```

3. Create `.env` file and put it in the `src-tauri` directory. Define your [API Ninjas](https://api-ninjas.com/api/loremipsum) API key.

```
API_KEY=INSERT-API-KEY-HERE
```