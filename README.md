# YorokoBot

This bot is a remake of the original HokuBot
using Rust with Serenity and the Nix package manager.

It is made for the [Archetype:Moon](https://archetype-moon.fr/) Discord server
exclusively and won't work elsewhere.

## What is YorokoBot ?

YorokoBot is a Discord bot allowing users to subscribe to tags.
Tags represent someone's center of interest.
If something related to this topic is post on Discord, the sender can use
the bot to ping each users that subscribed to this topic.

## Why using YorokoBot when Discord roles exist ?

To keep it simple, to prevent a Discord server to create a ton of roles.
Moreover, the goal is to keep it simple for Moderators with simple commands.

## Can everyone can add new tags ?

No, only server's admins can create new tags for their own server.

## Is everyone able to ping a tag (and possibly spam with it) ?

Well, this is something that is server dependant.
By default, only Discord admins will be able to ping a tag.
But they can change this behaviour with a simple command that will allow
each server member to ping someone.

## What if I want to block someone to use the bot on my server specifically ?

Each server will have a ban list at its disposition.
Server admins will be able to ban someone to use the bot on their server.

## Is there a global banlist ?

No, at least for now.
But it could be a future feature.

## Why is it designed to handle multiple servers ?

Because as a student, I can't afford a big infrastructure.
This bot will first be limited to a single server.
I will see to make it available for everyone later.
