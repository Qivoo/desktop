import React, { useState } from 'react';

interface Props { onSend: (text: string) => void; }

export function MessageInput({ onSend }: Props) {
  const [text, setText] = useState('');
  return (
    <form onSubmit={(e) => { e.preventDefault(); if (text.trim()) { onSend(text.trim()); setText(''); } }} style={{ display: 'flex', gap: '8px', padding: '12px', background: '#16213e' }}>
      <input value={text} onChange={(e) => setText(e.target.value)} placeholder="Type a message..." style={{ flex: 1, padding: '10px 14px', borderRadius: '8px', border: '1px solid #444', background: '#0f0f2a', color: '#fff' }} />
      <button type="submit" style={{ padding: '10px 16px', borderRadius: '8px', border: 'none', background: '#0f0f2a', color: '#fff', cursor: 'pointer' }}>Send</button>
    </form>
  );
}
