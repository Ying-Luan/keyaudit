import { invoke } from '@tauri-apps/api/core'
import { useState } from 'react'

type HotKeyStatus = "Occupied" | "System"

interface HotKey {
  modifiers: string[]
  key: string
  vk_code: number
  mod_flags: number
  status: HotKeyStatus
}

function App() {
  const [hotkeys, setHotkeys] = useState<HotKey[]>([])
  const [loading, setLoading] = useState(false)

  async function scan() {
    setLoading(true)
    const result = await invoke<HotKey[]>('scan')
    setHotkeys(result)
    setLoading(false)
  }

  return (
    <div>
      <button onClick={scan} disabled={loading}>
        {loading ? 'Scanning...' : 'Start Scan'}
      </button>
      {hotkeys.map((hk, i) => (
        <div key={i}>
          {hk.modifiers.join('+')}+{hk.key}
        </div>
      ))}
    </div>
  )
}

export default App
