# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Wampa is a Discord bot written in Rust using the Serenity framework. The bot manages new member onboarding for a Discord server by:
- Greeting new members when they join
- Processing name commands to set nicknames and assign roles
- Cleaning up welcome channel messages after role assignment

## Development Commands

### Building and Running
- `cargo build` - Build the project in debug mode
- `cargo build --release` - Build optimized release version
- `cargo run` - Build and run the bot locally
- `cargo check` - Fast syntax and type checking without building

### Testing
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run specific test

### Docker
- `docker build -t wampa .` - Build Docker image
- The project publishes to GitHub Container Registry automatically on pushes to master

## Architecture

### Core Components
- `main.rs` - Entry point, initializes Discord client with required intents
- `event_handler.rs` - Handles Discord events (ready, message, member_addition)
- `command.rs` - Command parsing and execution logic

### Event Flow
1. **Member Join**: `guild_member_addition` sends welcome message with instructions
2. **Command Processing**: `message` event parses commands starting with prefix (default "?")
3. **Name Command**: Sets user nickname, assigns roles, and cleans up messages

### Environment Variables
Required environment variables (see Dockerfile for defaults):
- `DISCORD_TOKEN` - Discord bot token
- `COMMAND_PREFIX` - Command prefix (default: "?")
- `WELCOME_CHANNEL_ID` - Channel ID for welcome messages
- `ROLE_CHANNEL_ID` - Channel ID mentioned in welcome message
- `MEMBER_ROLE_ID` - Role ID assigned to new members
- `ROOKIE_ROLE_ID` - Additional role ID assigned to new members
- `WELCOME_MESSAGE` - Custom welcome message template

### Dependencies
- `serenity` (0.12.2) - Discord API library
- `tokio` - Async runtime
- `dotenv` - Environment variable loading
- `chrono` - Date/time handling for message cleanup

## Key Implementation Details

### Message Cleanup Logic
The bot deletes messages in the welcome channel that are:
- Less than 14 days old
- Either mention the new user or are authored by the new user

### Error Handling
Custom `WampaError` enum handles:
- `InternalServerError` - System/library errors
- `InvalidCmd` - Command parsing errors

### Discord Intents
Requires specific gateway intents:
- `GUILD_MEMBERS` - For member join events
- `GUILDS` - For guild access
- `GUILD_MESSAGES` + `MESSAGE_CONTENT` - For command processing