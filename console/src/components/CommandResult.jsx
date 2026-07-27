import { useState } from 'react'

export default function CommandResult({ command }) {
  const [copied, setCopied] = useState(false)

  async function handleCopy() {
    if (!command?.output) return
    await navigator.clipboard.writeText(command.output)
    setCopied(true)
    setTimeout(() => setCopied(false), 1500)
  }

  return (
    <div className="command-result">
      <div className="panel-header panel-header-row">
        <span>OUTPUT</span>
        {command?.output && (
          <button className="copy-btn" onClick={handleCopy}>{copied ? 'copied' : 'copy'}</button>
        )}
      </div>
      <div className="result-content">
        {!command && <span className="dim">select a command</span>}
        {command && !command.output && <span className="dim">no output yet</span>}
        {command?.output && <pre>{command.output}</pre>}
      </div>
    </div>
  )
}
