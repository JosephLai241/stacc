         ___                         ___           ___           ___
        /\__\                       /\  \         /\__\         /\__\
       /:/ _/_         ___         /::\  \       /:/  /        /:/  /
      /:/ /\  \       /\__\       /:/\:\  \     /:/  /        /:/  /
     /:/ /::\  \     /:/  /      /:/ /::\  \   /:/  /  ___   /:/  /  ___
    /:/_/:/\:\__\   /:/__/      /:/_/:/\:\__\ /:/__/  /\__\ /:/__/  /\__\
    \:\/:/ /:/  /  /::\  \      \:\/:/  \/__/ \:\  \ /:/  / \:\  \ /:/  /
     \::/ /:/  /  /:/\:\  \      \::/__/       \:\  /:/  /   \:\  /:/  /
      \/_/:/  /   \/__\:\  \      \:\  \        \:\/:/  /     \:\/:/  /
        /:/  /         \:\__\      \:\__\        \::/  /       \::/  /
        \/__/           \/__/       \/__/         \/__/         \/__/

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/JosephLai241/stacc/deploy.yml?logo=github%20actions&label=Deploy)](https://github.com/JosephLai241/stacc/actions/workflows/deploy.yml)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/JosephLai241/stacc/rust.yml?logo=rust&logoColor=orange&label=Rust%20code%20checks)](https://github.com/JosephLai241/stacc/actions/workflows/rust.yml)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/JosephLai241/stacc/prettier.yml?logo=prettier&label=Prettier)](https://github.com/JosephLai241/stacc/actions/workflows/prettier.yml)
[![Docker Image Version (latest semver)](https://img.shields.io/docker/v/jlai241/stacc-api?logo=docker&label=Docker%20version)](https://hub.docker.com/repository/docker/jlai241/stacc-api/general)

# Table of Contents

- [What Is This?](#what-is-this)
- [The Stack](#the-stack)

# What Is This?

This is my full-stack portfolio site written entirely in Rust to prove the haters wrong -- Rust _is_ production ready.

I thought it would be cool to implement a dynamic background for the site. A new background GIF is loaded each time you refresh the homepage. This list of GIFs will continue to grow over the years as I find more GIFs that embody the right vibes. Enjoy cycling through the GIFs!

# The Stack

This project uses the following stack:

|          |                                                                                                                           |
| -------- | ------------------------------------------------------------------------------------------------------------------------- |
| Frontend | [`Yew`][yew]                                                                                                              |
| Backend  | [`Actix Web`][actix web]                                                                                                  |
| Database | [![MongoDB Badge](https://img.shields.io/badge/MongoDB-4EA94B?style=for-the-badge&logo=mongodb&logoColor=white)][mongodb] |

[yew]: https://yew.rs/
[actix web]: https://actix.rs/
[mongodb]: https://www.mongodb.com/
