use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::rngs::OsRng;
use crate::db::DbState;
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub status: Option<String>,
}

#[tauri::command]
pub fn register(
    state: tauri::State<DbState>,
    username: String,
    password: String,
) -> Result<AuthResponse, String> {
    let conn = state.0.lock().unwrap();
    
    // Hashage du mot de passe
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?
        .to_string();

    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now().to_rfc3339();
    
    // Le premier utilisateur devient admin automatiquement (pour toi), les suivants sont 'user'
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0)).unwrap_or(0);
    let (role, status) = if count == 0 {
        ("admin", "accepted")
    } else {
        ("user", "pending")
    };

    match conn.execute(
        "INSERT INTO users (id, username, password_hash, role, status, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        [&id, &username, &password_hash, role, status, &created_at],
    ) {
        Ok(_) => Ok(AuthResponse {
            success: true,
            message: if role == "admin" { "Compte Admin créé !".into() } else { "Demande envoyée à l'administrateur.".into() },
            status: Some(status.into()),
        }),
        Err(_) => Err("Ce nom d'utilisateur est déjà pris.".into()),
    }
}

#[tauri::command]
pub fn login(
    state: tauri::State<DbState>,
    username: String,
    password: String,
) -> Result<AuthResponse, String> {
    let conn = state.0.lock().unwrap();
    
    let mut stmt = conn
        .prepare("SELECT password_hash, status FROM users WHERE username = ?1")
        .map_err(|e| e.to_string())?;
    
    let (stored_hash, status): (String, String) = stmt
        .query_row([&username], |row| Ok((row.get(0)?, row.get(1)?)))
        .map_err(|_| "Utilisateur non trouvé.".to_string())?;

    // Vérification du mot de passe
    let parsed_hash = PasswordHash::new(&stored_hash).map_err(|e| e.to_string())?;
    let argon2 = Argon2::default();
    
    if argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok() {
        if status == "accepted" {
            // Créer la session dans la DB
            conn.execute(
                "INSERT OR REPLACE INTO sessions (id, user_id, username) VALUES (1, (SELECT id FROM users WHERE username = ?1), ?1)", 
                [&username]
            ).map_err(|e| e.to_string())?;

            Ok(AuthResponse {
                success: true,
                message: "Connexion réussie.".into(),
                status: Some(status),
            })
        } else if status == "pending" {
            Ok(AuthResponse {
                success: false,
                message: "Votre compte est en attente de validation par l'administrateur.".into(),
                status: Some(status),
            })
        } else {
            Ok(AuthResponse {
                success: false,
                message: "Votre demande a été refusée.".into(),
                status: Some(status),
            })
        }
    } else {
        Err("Mot de passe incorrect.".into())
    }
}

#[tauri::command]
pub fn logout(state: tauri::State<DbState>) -> Result<(), String> {
    let conn = state.0.lock().unwrap();
    conn.execute("DELETE FROM sessions WHERE id = 1", []).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_current_session(state: tauri::State<DbState>) -> Result<Option<String>, String> {
    let conn = state.0.lock().unwrap();
    let mut stmt = conn.prepare("SELECT username FROM sessions WHERE id = 1").map_err(|e| e.to_string())?;
    let username: Option<String> = stmt.query_row([], |row| row.get(0)).optional().map_err(|e| e.to_string())?;
    Ok(username)
}
