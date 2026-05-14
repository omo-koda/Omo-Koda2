import { AgentDashboard } from '@/components/AgentDashboard'
import { WalletProvider } from '@/components/WalletProvider'

export default function Home() {
  return (
    <WalletProvider>
      <main className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900">
        <div className="container mx-auto px-4 py-8">
          <header className="text-center mb-8">
            <h1 className="text-4xl font-bold text-white mb-2">Omo-Koda</h1>
            <p className="text-xl text-gray-300">Decentralized AI Agent Platform</p>
          </header>
          <AgentDashboard />
        </div>
      </main>
    </WalletProvider>
  )
}