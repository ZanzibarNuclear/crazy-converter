# Crazy Converterator

## Overview

Crazy Converterator is a tool for converting between any two things. It uses natural language processing to go from one thing to another along some dimension.

For example, someone might ask "How many horses would you need to replace the energy produced by a nuclear power plant?"

The Crazy Converterator asks a few clarifying questions, and uses tools to figure out the answer. The final answer would include a step-by-step explanation of how to get from the starting point to the end.

Crazy Converterator is for fun. While we want the conversions to have a sense of accuracy, they are not meant to represent real situations. We want to give people the freedom to compare any two things without getting hung up by what is possible or practical.

## Foundation

Crazy Converterator is built to use real conversion calculations. To that end, it relies on a variety of conversion tools that work across many dimensions: size, weight, volume, speed, time, energy, etc.

It uses a conversational interface backed by LLMs to interact with users.
Both command-line and web interfaces are supported. The tools include custom-built tools written in Rust for super-efficient performance, plus additional tools accessed using MCP.

Crazy Converterator is hosted in the Cloud. It also runs on local hardware with Internet access. It is built to run on Linux and UNIX (macOS).

## Development Milestones

### Phase 1: Common Conversions

1. Create a tool in Rust that handles basic conversions between common units. There is no creativity involved. We just want to support well known conversions and get the components connected.
2. Set up the chat interface in a browser, backed by FastAPI and Pydantic AI for interacting with LLMs.
3. Plug the conversion tool into the Pydantic AI backend.

### Phase 2: Bask in the Glory

That is all for now.
