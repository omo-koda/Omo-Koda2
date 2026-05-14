export interface Agent {
  id: string
  name: string
  dnaFingerprint: string
  primaryOdu: number
  birthTimestamp: number
  reputation: number
  tier: number
  owner: string
}

export interface ConversationMessage {
  role: 'user' | 'assistant'
  content: string
  timestamp: number
  isPrivate: boolean
}

export interface Tool {
  name: string
  description: string
  requiredTier: number
  category: 'web' | 'computation' | 'blockchain' | 'system'
}

export interface Provider {
  name: string
  class: 'local' | 'remote'
  endpoint?: string
  isAvailable: boolean
}

export interface SessionConfig {
  defaultProvider: string
  defaultPrivacy: boolean
  defaultSandbox: boolean
}

export interface ExecutionResult {
  receipt?: string
  toolOutput?: string
  privateMode: boolean
  error?: string
}