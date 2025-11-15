cd ~/coaching-flow

# Crear README.md con todas las versiones
cat > README.md << 'EOF'
# 🎓 Coaching Flow

Sistema de gestión de sesiones de coaching en Stellar Blockchain usando Soroban Smart Contracts.

## 📋 Descripción
Coaching Flow permite:
- 💰 Pagos en custodia (escrow) para sesiones de coaching
- 📊 Tracking de asistencia y estadísticas de clientes
- ⚠️ Detección de riesgo de abandono
- ✅ Liberación de pagos al completar sesiones

## 🛠️ Versiones de Herramientas Utilizadas
```bash
Node.js: v22.21.0
npm: 10.9.4
Rust: 1.90.0 (1159e78c4 2025-09-14)
Cargo: 1.90.0 (840b83a10 2025-07-30)
Stellar CLI: 23.2.0 (8c559e832fd969aa469784b66e70891fadf94f0a)
Stellar XDR: 23.0.0 (e83a6337204ecfdb0ac0d44ffb857130c1249b1b)
Git: 2.43.0
WSL: Ubuntu (22.04 o superior)
```

**Sistema Operativo:** Windows con WSL2 (Ubuntu)
**Wallet:** Freighter (extensión de navegador)

## 🏗️ Estructura del Proyecto
```
coaching-flow/
├── contracts/              # Smart Contracts en Rust
│   └── coaching-contract/
│       ├── src/
│       │   └── lib.rs     # Código del contrato
│       ├── Cargo.toml     # Dependencias
│       └── target/        # Compilados (WASM)
├── frontend/              # Interfaz web en React
│   ├── src/
│   │   ├── App.jsx       # Componente principal
│   │   └── App.css       # Estilos
│   └── package.json
└── scripts/              # Scripts de utilidad
```

## 🚀 Tecnologías

- **Blockchain:** Stellar (Soroban)
- **Contratos:** Rust + Soroban SDK
- **Frontend:** React + Vite
- **Wallet:** Freighter
- **Red:** Testnet

## 📦 Instalación Local

### Requisitos Previos

1. **Instalar Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown
```

2. **Instalar Soroban CLI:**
```bash
cargo install --locked soroban-cli --features opt
```

3. **Instalar Node.js:**
```bash
# Usando nvm (recomendado)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
source ~/.bashrc
nvm install --lts
```

4. **Instalar Freighter:**
   - Ir a [freighter.app](https://www.freighter.app/)
   - Agregar extensión al navegador
   - Crear cuenta en Testnet

### Clonar el Repositorio
```bash
git clone https://github.com/juliheit/Coaching-flow.git
cd Coaching-flow
```

### Compilar el Contrato
```bash
cd contracts/coaching-contract
cargo build --target wasm32-unknown-unknown --release
```

El archivo WASM estará en: `target/wasm32-unknown-unknown/release/coaching_contract.wasm`

### Configurar Frontend
```bash
cd ../../frontend
npm install
```

**Configurar la dirección del contrato:**
1. Abre `frontend/src/App.jsx`
2. En la línea 6, actualiza:
```javascript
const CONTRACT_ADDRESS = 'TU_CONTRATO_DESPLEGADO';
```

### Ejecutar Frontend Localmente
```bash
npm run dev
```

Abre http://localhost:5173 en tu navegador.

## 🌐 Ver el Proyecto en Línea (GitHub Pages)

Para que otros vean tu proyecto sin instalar nada:

### Opción 1: Desplegar en Vercel (Recomendado - Más Fácil)

1. Ve a [vercel.com](https://vercel.com)
2. Inicia sesión con GitHub
3. Click en "Add New" → "Project"
4. Selecciona el repositorio `Coaching-flow`
5. Configuración:
   - **Framework Preset:** Vite
   - **Root Directory:** `frontend`
   - **Build Command:** `npm run build`
   - **Output Directory:** `dist`
6. Click en "Deploy"

Tu proyecto estará en: `https://tu-proyecto.vercel.app`

## 🌐 Demo en Vivo

**Frontend desplegado:** [https://coaching-flow-sigma.vercel.app/](https://coaching-flow-sigma.vercel.app/)

Puedes interactuar con el contrato directamente desde el navegador:
1. Instala Freighter Wallet
2. Crea/importa cuenta en Testnet
3. Conecta tu wallet
4. Consulta sesiones y estadísticas

## 🌐 Información de Despliegue

- **Contrato Testnet:** `CB6Z6Y7QYZD2WDF3MYD7GHQSXDWVHYDQWBR6XTG3L2UL2FGPHMFCC5X7`
- **Token de Pago:** XLM Nativo
- **Red:** Stellar Testnet
- **Explorer:** [Stellar Expert](https://stellar.expert/explorer/testnet)
- **Frontend:** [Vercel](https://coaching-flow-sigma.vercel.app/)
```

Tu proyecto estará en: `https://juliheit.github.io/Coaching-flow/`

## 🔧 Despliegue en Stellar Testnet

### Configurar Red Testnet
```bash
soroban network add \
  --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
```

### Crear Cuentas
```bash
# Crear identidades
soroban keys generate alice --network testnet
soroban keys generate client --network testnet
soroban keys generate coach --network testnet

# Fondear cuentas
soroban keys fund alice --network testnet
soroban keys fund client --network testnet
soroban keys fund coach --network testnet
```

### Desplegar Contrato
```bash
cd contracts/coaching-contract

# Desplegar
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/coaching_contract.wasm \
  --source alice \
  --network testnet

# Guardar la dirección del contrato que aparece
```

### Inicializar Contrato
```bash
# Usar XLM nativo como token de pago
soroban contract invoke \
  --id TU_CONTRATO_ADDRESS \
  --source alice \
  --network testnet \
  -- \
  initialize \
  --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC
```

## 📖 Uso del Contrato

### Crear una Sesión
```bash
soroban contract invoke \
  --id CONTRACT_ADDRESS \
  --source client \
  --network testnet \
  -- \
  create_session \
  --client CLIENT_ADDRESS \
  --coach COACH_ADDRESS \
  --amount 10000000 \
  --scheduled_time 1700000000
```

### Marcar Asistencia
```bash
soroban contract invoke \
  --id CONTRACT_ADDRESS \
  --source coach \
  --network testnet \
  -- \
  mark_attendance \
  --session_id 1 \
  --attended true
```

### Completar Sesión
```bash
soroban contract invoke \
  --id CONTRACT_ADDRESS \
  --source coach \
  --network testnet \
  -- \
  complete_session \
  --session_id 1
```

### Consultar Sesión
```bash
soroban contract invoke \
  --id CONTRACT_ADDRESS \
  --source alice \
  --network testnet \
  -- \
  get_session \
  --session_id 1
```

### Ver Estadísticas
```bash
soroban contract invoke \
  --id CONTRACT_ADDRESS \
  --source alice \
  --network testnet \
  -- \
  get_client_stats \
  --client CLIENT_ADDRESS
```

## 📚 Funciones del Contrato

### `initialize(token: Address)`
Inicializa el contrato con el token de pago (solo una vez).

### `create_session(client, coach, amount, scheduled_time) -> u64`
- Crea nueva sesión
- Bloquea el pago en el contrato
- Retorna el ID de la sesión

### `mark_attendance(session_id, attended: bool)`
- Solo el coach puede llamarla
- Marca si el cliente asistió

### `complete_session(session_id)`
- Solo el coach puede llamarla
- Requiere que attended = true
- Libera el pago al coach

### `get_session(session_id) -> CoachingSession`
Consulta información de una sesión.

### `get_client_stats(client) -> ClientStats`
Obtiene estadísticas y riesgo de abandono.

## 🎯 Lógica de Riesgo de Abandono

Un cliente está marcado como "en riesgo" si:
- Ha faltado a 2 o más sesiones consecutivas, O
- Su tasa de asistencia es menor al 50% (con mínimo 3 sesiones)

## 🌐 Información de Despliegue

- **Contrato Testnet:** `CB6Z6Y7QYZD2WDF3MYD7GHQSXDWVHYDQWBR6XTG3L2UL2FGPHMFCC5X7`
- **Token de Pago:** XLM Nativo
- **Red:** Stellar Testnet
- **Explorer:** [Stellar Expert](https://stellar.expert/explorer/testnet)

## 🧪 Testing
```bash
cd contracts/coaching-contract
cargo test
```

## 📝 Estructura del Contrato
```rust
// Estructuras principales
CoachingSession {
    session_id: u64,
    client: Address,
    coach: Address,
    amount: i128,
    scheduled_time: u64,
    attended: bool,
    completed: bool,
}

ClientStats {
    total_sessions: u64,
    attended_sessions: u64,
    missed_sessions: u64,
    at_risk: bool,
}
```

## 🔐 Seguridad

- ✅ Autenticación con `require_auth()`
- ✅ Validaciones de estado (no se puede completar sin asistencia)
- ✅ Pagos en custodia (escrow)
- ✅ Una sola inicialización permitida

## 🤝 Contribuciones

Este es un proyecto de práctica para aprender Soroban. Si encuentras mejoras, ¡son bienvenidas!

## 📄 Licencia

MIT

## 👤 Autor

Proyecto desarrollado como práctica de Soroban Smart Contracts en Stellar.

---

**Nota:** Este proyecto está desplegado en Testnet únicamente para fines educativos.

