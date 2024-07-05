# Projet d'Automatisation de Ferme à Troll

## Vue d'ensemble

Ce projet est conçu pour créer un système automatisé de génération et de gestion de comptes Twitter, avec la possibilité de publier automatiquement des tweets. L'objectif principal est soit d'inonder les réseaux sociaux de désinformation, soit de diffuser des informations positives, selon les objectifs. La phase initiale implique la création d'un créateur de compte automatisé, suivie par la génération et l'upload de messages via l'API Twitter avec l'aide de l'IA.

## Prérequis

Avant de commencer à utiliser ce projet, assurez-vous d'avoir les prérequis suivants installés et configurés sur votre système :

1. **Rust** : Assurez-vous d'avoir le langage de programmation Rust installé. Vous pouvez le télécharger [ici](https://www.rust-lang.org/tools/install).

2. **Chrome Driver** : Ce projet utilise la crate `thirtyfour` pour contrôler le navigateur Chrome via WebDriver. Vous devez avoir ChromeDriver installé et en cours d'exécution. Vous pouvez le télécharger [ici](https://sites.google.com/a/chromium.org/chromedriver/downloads).

3. **Google Chrome** : Assurez-vous d'avoir Google Chrome installé sur votre système, car ChromeDriver contrôlera ce navigateur.

## Installation et Configuration

1. **Cloner le Répertoire**

   ```sh
   git clone https://github.com/Cholulaa/Projet-rust-Troll-Farm.git
   cd Projet-rust-Troll-Farm
   ```

2. **Exécuter ChromeDriver**

   Téléchargez la version appropriée de ChromeDriver pour votre système d'exploitation depuis le [site officiel](https://sites.google.com/a/chromium.org/chromedriver/downloads) et exécutez-le :

   ```sh
   ./chromedriver
   ```

   Assurez-vous qu'il fonctionne sur le port par défaut `9515`.

3. **Exécuter le Projet**

   ```sh
   cargo run
   ```

## Détails du Projet

### Créateur de Compte Automatisé

La phase initiale de ce projet inclut un créateur de compte automatisé qui navigue vers la page d'inscription de Twitter, remplit les détails nécessaires et crée un nouveau compte. Cela implique :

- Cliquer sur le bouton "Créer un compte"
- Remplir le champ "Nom et prénom"
- Remplir le champ "Email"
- Sélectionner la date de naissance (mois, jour, année)
- Cliquer sur les boutons "Suivant" pour continuer le processus d'inscription

### Améliorations Futures

Les prochaines étapes de ce projet incluront :

1. **Génération et Publication de Tweets** : Utilisation de l'API Twitter pour générer et publier automatiquement des tweets. Cela pourrait impliquer l'intégration de l'IA pour générer le contenu des messages.

2. **Intégration de l'IA** : Mise en œuvre de l'IA pour générer des messages qui diffusent soit de la désinformation, soit des informations positives, selon les objectifs du projet.

## Utilisation

Le projet exécute actuellement un script qui automatise la création d'un compte Twitter. Les versions futures incluront plus de fonctionnalités pour tweeter automatiquement.

---

En suivant ces instructions, vous devriez être en mesure de configurer et d'exécuter le Projet de Ferme à Troll, en commençant par la création automatisée de comptes Twitter et en avançant vers des fonctionnalités plus avancées.