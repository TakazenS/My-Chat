import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import styles from "./Auth.module.css";

interface AuthProps {
  onLoginSuccess: (username: string) => void;
}

interface AuthResponse {
  success: boolean;
  message: string;
  status?: string;
}

export default function Auth({ onLoginSuccess }: AuthProps) {
  const [isRegistering, setIsRegistering] = useState(false);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [message, setMessage] = useState("");

  async function handleRegister() {
    try {
      const res: AuthResponse = await invoke("register", { username, password });
      setMessage(res.message);
      if (res.success && res.status === "accepted") {
        onLoginSuccess(username);
      }
    } catch (err: any) {
      setMessage(err);
    }
  }

  async function handleLogin() {
    try {
      const res: AuthResponse = await invoke("login", { username, password });
      if (res.success) {
        onLoginSuccess(username);
      } else {
        setMessage(res.message);
      }
    } catch (err: any) {
      setMessage(err);
    }
  }

  return (
    <div className={styles.authContainer}>
      <div className={styles.authBox}>
        <h1>{isRegistering ? "Créer un compte" : "Connexion"}</h1>
        <input
          className={styles.input}
          placeholder="Nom d'utilisateur"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
        />
        <div className={styles.passwordWrapper}>
          <input
            className={styles.input}
            type={showPassword ? "text" : "password"}
            placeholder="Mot de passe"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
          {password.length > 0 && (
            <div className={styles.eyeIcon} onClick={() => setShowPassword(!showPassword)}>
              {showPassword ? (
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"></path><line x1="1" y1="1" x2="23" y2="23"></line></svg>
              ) : (
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path><circle cx="12" cy="12" r="3"></circle></svg>
              )}
            </div>
          )}
        </div>
        <button className={styles.button} onClick={isRegistering ? handleRegister : handleLogin}>
          {isRegistering ? "S'inscrire" : "Se connecter"}
        </button>
        <p className={styles.toggleText} onClick={() => setIsRegistering(!isRegistering)}>
          {isRegistering ? "Déjà un compte ? Se connecter" : "Besoin d'un compte ? S'inscrire"}
        </p>
        {message && <div className={styles.message}>{message}</div>}
      </div>
    </div>
  );
}
