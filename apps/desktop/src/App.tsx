import React from 'react';
import { MessageList } from './components/MessageList';
import { MessageInput } from './components/MessageInput';
import { ModelStatus } from './components/ModelStatus';
import { useAgent } from './state/AgentStore';

export default function App() {
  const { messages, status, sendMessage, approveTool, cancelTurn } = useAgent();

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100vh', background: '#1a1a2e' }}>
      <header style={{ padding: '12px 20px', borderBottom: '1px solid #333', background: '#16213e' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
          <h1 style={{ color: '#fff', margin: 0, fontSize: '18px' }}>Local-First AI Agent</h1>
          <ModelStatus status={status} />
        </div>
      </header>
      <main style={{ flex: 1, overflowY: 'auto', padding: '16px', display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <MessageList messages={messages} />
        <MessageInput onSend={sendMessage} />
      </main>
    </div>
  );
}
