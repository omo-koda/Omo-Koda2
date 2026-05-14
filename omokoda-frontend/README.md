# Omo-Koda Frontend

This directory contains the Next.js frontend skeleton for the Omo-Koda platform.

## Overview

The frontend provides a simple agent dashboard with the following sections:

- Chat interface for agent interaction
- Agent profile summary
- Tool catalog and tier listing
- Platform statistics overview

## Setup

Install dependencies:

```bash
cd omokoda-frontend
npm install
```

Run the development server:

```bash
npm run dev
```

Build for production:

```bash
npm run build
```

## Architecture

- `app/page.tsx` - main landing page for the application
- `app/layout.tsx` - base layout and metadata
- `components/AgentDashboard.tsx` - dashboard browser with tabs
- `components/AgentChat.tsx` - agent chat panel
- `components/AgentProfile.tsx` - profile and reputation summary
- `components/AgentTools.tsx` - registered tool catalog
- `components/AgentStats.tsx` - platform metrics overview
- `app/api/chat/route.ts` - placeholder chat API route

## Notes

The current implementation is a skeleton intended to integrate with the Omo-Koda backend and the Sui Move contract layer.
