import styles from "./Settings.module.css";

interface SettingsProps {
  onBack: () => void;
  username: string;
}

export default function Settings({ onBack, username }: SettingsProps) {
  return (
    <div className={styles.container}>
      <h1 className={styles.header}>Paramètres</h1>
      <p>Bienvenue dans vos paramètres, {username}.</p>
      {/* Nous ajouterons les formulaires de modification ici plus tard */}
      <button className={styles.backButton} onClick={onBack}>Retour</button>
    </div>
  );
}
