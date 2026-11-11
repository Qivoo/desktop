import React from 'react';
import { ModelMessage } from '../types';

interface Props { messages: ModelMessage[]; }

export function MessageList({ messages }: Props) {
  return (
    <div style={{ flex: 1, display: 'flex', flexDirection: 'column', gap: '12px', overflowY: 'auto' }}>
      {messages.map((msg) => (
        <div key={msg.id} style={{ maxWidth: '70%', padding: '10px 14px', borderRadius: '10px', background: msg.role === 'assistant' ? '#16213e' : msg.role === 'user' ? '#2a2a4a' : '#2a3a1a' }}>
          <div style={{ color: '#fff', whiteSpace: 'pre-wrap' }}>{msg.content}</div>
          {msg.toolCalls && <div style={{ marginTop: '8px', background: '#333', borderRadius: '6px', padding: '8px', fontFamily: 'monospace', fontSize: '12px', color: '#ddd' }}>TOOL: {msg.toolCalls.map(t => t.name).join(', ')}</div>}
          {msg.toolResult && <div style={{ marginTop: '8px', background: '#1a3a1a', borderRadius: '6px', padding: '8px', fontFamily: 'monospace', fontSize: '12px', color: '#9f9' }}>RESULT: {msg.toolResult}</div>}
          {msg.status === 'pending' && <div style={{ color: '#888', fontSize: '12px', fontStyle: 'italic' }}>Thinking...</div>}
        </div>
      ))}
    </div>
  );
}
