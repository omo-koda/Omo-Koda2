import { NextResponse } from 'next/server'

export async function POST(request: Request) {
  const body = await request.json()
  const message = body?.message ?? ''

  // Placeholder response: this endpoint will be connected to Omo-Koda's backend
  const response = {
    response: `Received message: ${message}. Chat integration is coming soon.`,
  }

  return NextResponse.json(response)
}
