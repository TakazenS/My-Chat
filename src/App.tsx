import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import Auth from "./views/Auth/Auth";
import Main from "./views/Main/Main";
import "./App.css";

export default function App() {
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [username, setUsername] = useState("");
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function checkSession() {
      try {
        const storedUsername: string | null = await invoke("get_current_session");
        if (storedUsername) {
          setUsername(storedUsername);
          setIsLoggedIn(true);
        }
      } catch (err) {
        console.error("Erreur de session:", err);
      } finally {
        setIsLoading(false);
      }
    }
    checkSession();
  }, []);

  const handleLoginSuccess = (user: string) => {
    setUsername(user);
    setIsLoggedIn(true);
  };

  const handleLogout = () => {
    setIsLoggedIn(false);
    setUsername("");
  };

  if (isLoading) {
    return <div className="loadingContainer">Chargement...</div>;
  }

  return (
    <>
      {!isLoggedIn ? (
        <Auth onLoginSuccess={handleLoginSuccess} />
      ) : (
        <Main username={username} onLogout={handleLogout} />
      )}
    </>
  );
}
