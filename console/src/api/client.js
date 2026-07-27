const BASE = '/api'
const JSON_HEADERS = { 'Content-Type': 'application/json' }

export async function getCommands() {
  const res = await fetch(`${BASE}/commands/`)
  if (!res.ok) throw new Error('Failed to fetch commands')
  return res.json()
}

export async function createCommand(command, order) {
  const res = await fetch(`${BASE}/commands/`, {
    method: 'POST',
    headers: JSON_HEADERS,
    body: JSON.stringify({ command, order })
  })
  if (!res.ok) throw new Error('Failed to create command')
  return res.json()
}

export async function deleteCommand(id) {
  const res = await fetch(`${BASE}/commands/${id}`, {
    method: 'DELETE'
  })
  if (!res.ok) throw new Error('Failed to delete command')
}
