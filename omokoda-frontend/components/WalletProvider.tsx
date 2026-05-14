'use client'

import { WalletProvider as SuiWalletProvider } from '@suiet/wallet-kit'
import '@suiet/wallet-kit/style.css'

interface WalletProviderProps {
  children: React.ReactNode
}

export function WalletProvider({ children }: WalletProviderProps) {
  return (
    <SuiWalletProvider>
      {children}
    </SuiWalletProvider>
  )
}