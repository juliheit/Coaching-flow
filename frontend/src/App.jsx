import { useState, useEffect } from 'react';
import * as freighter from '@stellar/freighter-api';
import './App.css';

// CONFIGURACIÓN - Reemplaza con tus direcciones reales
const CONTRACT_ADDRESS = 'CB6Z6Y7QYZD2WDF3MYD7GHQSXDWVHYDQWBR6XTG3L2UL2FGPHMFCC5X7';
const NETWORK_PASSPHRASE = 'Test SDF Network ; September 2015';
const RPC_URL = 'https://soroban-testnet.stellar.org:443';

function App() {
  const [publicKey, setPublicKey] = useState('');
  const [connected, setConnected] = useState(false);
  const [sessionId, setSessionId] = useState('');
  const [sessionData, setSessionData] = useState(null);
  const [clientStats, setClientStats] = useState(null);
  const [loading, setLoading] = useState(false);

  // Conectar Freighter - VERSIÓN CORREGIDA
  const connectFreighter = async () => {
    try {
      const isConnectedToFreighter = await freighter.isConnected();
      if (!isConnectedToFreighter) {
        alert('Por favor instala Freighter extension');
        return;
      }
      
      // Solicitar acceso
      await freighter.requestAccess();
      
      // Obtener la clave pública
      const { address } = await freighter.getAddress();
      
      setPublicKey(address);
      setConnected(true);
      alert(`Conectado: ${address.substring(0, 8)}...`);
    } catch (error) {
      console.error('Error conectando Freighter:', error);
      alert('Error conectando con Freighter: ' + error.message);
    }
  };

  // Obtener información de una sesión
  const getSession = async () => {
    if (!sessionId) {
      alert('Ingresa un Session ID');
      return;
    }

    setLoading(true);
    try {
      // Aquí harías la llamada real al contrato
      // Por ahora mostramos datos de ejemplo
      const mockData = {
        session_id: parseInt(sessionId),
        client: "GAX63FSGPPYSD6ZTOZ7VMVEVHSIMFMW573UXHZGW5UFXQZDELCV37X5I",
        coach: "GC4OTBH4ZDDWVXORRUAZI2GFWDZ2UVRBBFROA7KMC5EKBJR7QEZL6D63",
        amount: "10000000",
        scheduled_time: 1700000000,
        attended: true,
        completed: true
      };
      
      setSessionData(mockData);
      alert('Sesión obtenida (datos de ejemplo)');
    } catch (error) {
      console.error('Error:', error);
      alert('Error obteniendo sesión');
    }
    setLoading(false);
  };

  // Obtener estadísticas del cliente
  const getStats = async () => {
    if (!publicKey) {
      alert('Conecta tu wallet primero');
      return;
    }

    setLoading(true);
    try {
      // Aquí harías la llamada real al contrato
      const mockStats = {
        total_sessions: 1,
        attended_sessions: 1,
        missed_sessions: 0,
        at_risk: false
      };
      
      setClientStats(mockStats);
      alert('Estadísticas obtenidas (datos de ejemplo)');
    } catch (error) {
      console.error('Error:', error);
      alert('Error obteniendo estadísticas');
    }
    setLoading(false);
  };

  return (
    <div className="App">
      <header>
        <h1>🎓 Coaching Flow</h1>
        <p>Sistema de Gestión de Sesiones de Coaching</p>
      </header>

      <main>
        {/* Sección de Conexión */}
        <section className="card">
          <h2>🔐 Conexión</h2>
          {!connected ? (
            <button onClick={connectFreighter} className="btn-primary">
              Conectar Freighter
            </button>
          ) : (
            <div className="connected-info">
              <p>✅ Conectado</p>
              <p className="address">{publicKey}</p>
            </div>
          )}
        </section>

        {/* Información del Contrato */}
        <section className="card">
          <h2>📋 Información del Contrato</h2>
          <div className="info-grid">
            <div className="info-item">
              <strong>Dirección del Contrato:</strong>
              <p className="mono">{CONTRACT_ADDRESS}</p>
            </div>
            <div className="info-item">
              <strong>Red:</strong>
              <p>Testnet</p>
            </div>
          </div>
        </section>

        {/* Consultar Sesión */}
        <section className="card">
          <h2>🔍 Consultar Sesión</h2>
          <div className="form-group">
            <input
              type="number"
              placeholder="Session ID (ej: 1)"
              value={sessionId}
              onChange={(e) => setSessionId(e.target.value)}
              className="input"
            />
            <button 
              onClick={getSession} 
              disabled={loading}
              className="btn-primary"
            >
              {loading ? 'Cargando...' : 'Obtener Sesión'}
            </button>
          </div>

          {sessionData && (
            <div className="result-box">
              <h3>Información de la Sesión</h3>
              <div className="data-grid">
                <div><strong>ID:</strong> {sessionData.session_id}</div>
                <div><strong>Cliente:</strong> {sessionData.client}</div>
                <div><strong>Coach:</strong> {sessionData.coach}</div>
                <div><strong>Monto:</strong> {(sessionData.amount / 10000000).toFixed(1)} XLM</div>
                <div><strong>Asistió:</strong> {sessionData.attended ? '✅ Sí' : '❌ No'}</div>
                <div><strong>Completada:</strong> {sessionData.completed ? '✅ Sí' : '⏳ Pendiente'}</div>
              </div>
            </div>
          )}
        </section>

        {/* Estadísticas del Cliente */}
        <section className="card">
          <h2>📊 Mis Estadísticas</h2>
          <button 
            onClick={getStats} 
            disabled={loading || !connected}
            className="btn-primary"
          >
            {loading ? 'Cargando...' : 'Ver Mis Estadísticas'}
          </button>

          {clientStats && (
            <div className="result-box">
              <h3>Tu Historial</h3>
              <div className="stats-grid">
                <div className="stat-card">
                  <div className="stat-number">{clientStats.total_sessions}</div>
                  <div className="stat-label">Sesiones Totales</div>
                </div>
                <div className="stat-card">
                  <div className="stat-number">{clientStats.attended_sessions}</div>
                  <div className="stat-label">Asistencias</div>
                </div>
                <div className="stat-card">
                  <div className="stat-number">{clientStats.missed_sessions}</div>
                  <div className="stat-label">Inasistencias</div>
                </div>
                <div className={`stat-card ${clientStats.at_risk ? 'at-risk' : 'safe'}`}>
                  <div className="stat-number">{clientStats.at_risk ? '⚠️' : '✅'}</div>
                  <div className="stat-label">
                    {clientStats.at_risk ? 'En Riesgo' : 'Sin Riesgo'}
                  </div>
                </div>
              </div>
            </div>
          )}
        </section>
      </main>

      <footer>
        <p>Coaching Flow - Sistema de Gestión de Sesiones en Stellar</p>
      </footer>
    </div>
  );
}

export default App;