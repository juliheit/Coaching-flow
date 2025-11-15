// lib.rs - Contrato de Coaching Flow
// Este contrato gestiona sesiones de coaching y hace seguimiento de asistencia

// ============================================
// IMPORTACIONES
// ============================================
// soroban_sdk: Biblioteca principal para escribir contratos en Stellar
#![no_std] // No usamos la biblioteca estándar de Rust (para contratos ligeros)

use soroban_sdk::{
    contract,      // Macro para definir el contrato
    contractimpl,  // Macro para implementar funciones del contrato
    contracttype,  // Macro para definir tipos de datos personalizados
    token,         // Para interactuar con tokens (pagos)
    Address,       // Tipo para direcciones de usuarios/contratos
    Env,           // Entorno de ejecución del contrato
    Symbol,        // Tipo para identificadores de texto
    Vec,           // Vector (lista) de elementos
};

// ============================================
// DEFINICIÓN DE ESTRUCTURAS DE DATOS
// ============================================

// Estructura que representa una sesión de coaching
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoachingSession {
    pub session_id: u64,        // ID único de la sesión
    pub client: Address,        // Dirección del cliente
    pub coach: Address,         // Dirección del coach
    pub amount: i128,           // Monto pagado (en unidades del token)
    pub scheduled_time: u64,    // Timestamp de cuándo está programada
    pub attended: bool,         // ¿El cliente asistió?
    pub completed: bool,        // ¿La sesión se completó?
}

// Estructura para las estadísticas de un cliente
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientStats {
    pub total_sessions: u64,     // Total de sesiones reservadas
    pub attended_sessions: u64,  // Sesiones a las que asistió
    pub missed_sessions: u64,    // Sesiones que no asistió
    pub at_risk: bool,           // ¿Tiene riesgo de abandono?
}

// ============================================
// DEFINICIÓN DE CLAVES DE ALMACENAMIENTO
// ============================================
// Usamos símbolos para identificar datos en el almacenamiento del contrato

const SESSION_COUNT: Symbol = Symbol::short("SESSCOUNT"); // Contador de sesiones
const SESSION_PREFIX: Symbol = Symbol::short("SESSION");   // Prefijo para sesiones
const STATS_PREFIX: Symbol = Symbol::short("STATS");       // Prefijo para estadísticas
const TOKEN_ADDRESS: Symbol = Symbol::short("TOKEN");      // Dirección del token de pago

// ============================================
// CONTRATO PRINCIPAL
// ============================================

#[contract]
pub struct CoachingContract;

#[contractimpl]
impl CoachingContract {
    
    // ========================================
    // FUNCIÓN: Inicializar el contrato
    // ========================================
    // Se ejecuta UNA SOLA VEZ al desplegar el contrato
    // Parámetros:
    //   - env: entorno de ejecución
    //   - token: dirección del token que se usará para pagos (ej: USDC)
    pub fn initialize(env: Env, token: Address) {
        // Verificar que no se haya inicializado antes
        if env.storage().instance().has(&TOKEN_ADDRESS) {
            panic!("Contract already initialized");
        }
        
        // Guardar la dirección del token
        env.storage().instance().set(&TOKEN_ADDRESS, &token);
        
        // Inicializar el contador de sesiones en 0
        env.storage().instance().set(&SESSION_COUNT, &0u64);
    }
    
    // ========================================
    // FUNCIÓN: Crear una nueva sesión
    // ========================================
    // El cliente reserva y paga por una sesión de coaching
    // Parámetros:
    //   - client: dirección del cliente que reserva
    //   - coach: dirección del coach
    //   - amount: cantidad a pagar
    //   - scheduled_time: timestamp de cuándo será la sesión
    pub fn create_session(
        env: Env,
        client: Address,
        coach: Address,
        amount: i128,
        scheduled_time: u64,
    ) -> u64 {
        // AUTENTICACIÓN: Verificar que quien llama es el cliente
        client.require_auth();
        
        // Obtener dirección del token de pago
        let token_address: Address = env
            .storage()
            .instance()
            .get(&TOKEN_ADDRESS)
            .expect("Token not initialized");
        
        // PAGO: Transferir tokens del cliente al contrato
        // El contrato guarda los fondos hasta que la sesión se complete
        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(
            &client,                          // De: cliente
            &env.current_contract_address(),  // A: este contrato
            &amount                           // Cantidad
        );
        
        // CREAR SESIÓN: Obtener ID único para la nueva sesión
        let session_count: u64 = env
            .storage()
            .instance()
            .get(&SESSION_COUNT)
            .unwrap_or(0);
        
        let session_id = session_count + 1;
        
        // Crear estructura de la sesión
        let session = CoachingSession {
            session_id,
            client: client.clone(),
            coach: coach.clone(),
            amount,
            scheduled_time,
            attended: false,    // Inicialmente no ha asistido
            completed: false,   // Inicialmente no está completada
        };
        
        // ALMACENAR: Guardar la sesión en el storage
        // Usamos una clave compuesta: "SESSION" + ID
        let session_key = (SESSION_PREFIX, session_id);
        env.storage().persistent().set(&session_key, &session);
        
        // Actualizar contador de sesiones
        env.storage().instance().set(&SESSION_COUNT, &session_id);
        
        // ACTUALIZAR ESTADÍSTICAS del cliente
        Self::update_client_stats(&env, &client, true, false);
        
        // Retornar el ID de la sesión creada
        session_id
    }
    
    // ========================================
    // FUNCIÓN: Marcar asistencia
    // ========================================
    // El coach confirma que el cliente asistió a la sesión
    pub fn mark_attendance(env: Env, session_id: u64, attended: bool) {
        // Obtener la sesión
        let session_key = (SESSION_PREFIX, session_id);
        let mut session: CoachingSession = env
            .storage()
            .persistent()
            .get(&session_key)
            .expect("Session not found");
        
        // AUTENTICACIÓN: Solo el coach puede marcar asistencia
        session.coach.require_auth();
        
        // Actualizar estado de asistencia
        session.attended = attended;
        
        // Guardar cambios
        env.storage().persistent().set(&session_key, &session);
        
        // Actualizar estadísticas del cliente
        Self::update_client_stats(&env, &session.client, false, attended);
    }
    
    // ========================================
    // FUNCIÓN: Completar sesión y liberar pago
    // ========================================
    // Cuando la sesión termina, se paga al coach
    pub fn complete_session(env: Env, session_id: u64) {
        // Obtener la sesión
        let session_key = (SESSION_PREFIX, session_id);
        let mut session: CoachingSession = env
            .storage()
            .persistent()
            .get(&session_key)
            .expect("Session not found");
        
        // AUTENTICACIÓN: Solo el coach puede completar la sesión
        session.coach.require_auth();
        
        // VALIDACIÓN: La sesión debe estar marcada como atendida
        if !session.attended {
            panic!("Session must be marked as attended before completion");
        }
        
        // VALIDACIÓN: No se puede completar dos veces
        if session.completed {
            panic!("Session already completed");
        }
        
        // Obtener token de pago
        let token_address: Address = env
            .storage()
            .instance()
            .get(&TOKEN_ADDRESS)
            .expect("Token not initialized");
        
        // PAGO AL COACH: Transferir fondos del contrato al coach
        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(
            &env.current_contract_address(),  // De: este contrato
            &session.coach,                   // A: el coach
            &session.amount                   // Cantidad completa
        );
        
        // Marcar sesión como completada
        session.completed = true;
        env.storage().persistent().set(&session_key, &session);
    }
    
    // ========================================
    // FUNCIÓN: Obtener información de una sesión
    // ========================================
    pub fn get_session(env: Env, session_id: u64) -> CoachingSession {
        let session_key = (SESSION_PREFIX, session_id);
        env.storage()
            .persistent()
            .get(&session_key)
            .expect("Session not found")
    }
    
    // ========================================
    // FUNCIÓN: Obtener estadísticas de un cliente
    // ========================================
    // Permite ver el historial y riesgo de abandono
    pub fn get_client_stats(env: Env, client: Address) -> ClientStats {
        let stats_key = (STATS_PREFIX, client);
        env.storage()
            .persistent()
            .get(&stats_key)
            .unwrap_or(ClientStats {
                total_sessions: 0,
                attended_sessions: 0,
                missed_sessions: 0,
                at_risk: false,
            })
    }
    
    // ========================================
    // FUNCIÓN INTERNA: Actualizar estadísticas
    // ========================================
    // Esta función es privada (no se puede llamar desde fuera)
    fn update_client_stats(
        env: &Env,
        client: &Address,
        new_session: bool,
        attended: bool,
    ) {
        let stats_key = (STATS_PREFIX, client.clone());
        
        // Obtener estadísticas actuales o crear nuevas
        let mut stats: ClientStats = env
            .storage()
            .persistent()
            .get(&stats_key)
            .unwrap_or(ClientStats {
                total_sessions: 0,
                attended_sessions: 0,
                missed_sessions: 0,
                at_risk: false,
            });
        
        // Actualizar contadores
        if new_session {
            stats.total_sessions += 1;
        } else if attended {
            stats.attended_sessions += 1;
        } else {
            stats.missed_sessions += 1;
        }
        
        // LÓGICA DE RIESGO DE ABANDONO:
        // Un cliente está "en riesgo" si:
        // - Ha faltado a 2+ sesiones consecutivas, O
        // - Su tasa de asistencia es menor al 50% (con al menos 3 sesiones)
        if stats.missed_sessions >= 2 {
            stats.at_risk = true;
        } else if stats.total_sessions >= 3 {
            let attendance_rate = (stats.attended_sessions * 100) / stats.total_sessions;
            stats.at_risk = attendance_rate < 50;
        }
        
        // Guardar estadísticas actualizadas
        env.storage().persistent().set(&stats_key, &stats);
    }
}

// ============================================
// TESTS (Pruebas Unitarias)
// ============================================
// Estas pruebas se ejecutan con: cargo test

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, token, Env};

    #[test]
    fn test_create_and_complete_session() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CoachingContract);
        let client = CoachingContract::new(&env, &contract_id);

        // Crear direcciones de prueba
        let client_addr = Address::generate(&env);
        let coach_addr = Address::generate(&env);
        let token_admin = Address::generate(&env);

        // Crear token de prueba
        let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
        let token = token::Client::new(&env, &token_contract.address());
        let token_admin_client = token::StellarAssetClient::new(&env, &token_contract.address());

        // Dar tokens al cliente
        token_admin_client.mint(&client_addr, &1000);

        // Inicializar contrato
        client.initialize(&token_contract.address());

        // Crear sesión
        let session_id = client.create_session(
            &client_addr,
            &coach_addr,
            &100,
            &1000000,
        );

        assert_eq!(session_id, 1);

        // Marcar asistencia
        client.mark_attendance(&session_id, &true);

        // Completar sesión
        client.complete_session(&session_id);

        // Verificar que el coach recibió el pago
        assert_eq!(token.balance(&coach_addr), 100);
    }
}